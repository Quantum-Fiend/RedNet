"""
Encoding and Obfuscation Module
Provides various encoding schemes for payload transformation
"""

import base64
from abc import ABC, abstractmethod
from typing import List
import random


class Encoder(ABC):
    """Base encoder class"""
    
    @abstractmethod
    def encode(self, data: bytes) -> bytes:
        """Encode data"""
        pass
    
    @abstractmethod
    def decode(self, data: bytes) -> bytes:
        """Decode data"""
        pass


class XorEncoder(Encoder):
    """XOR encoding with a key"""
    
    def __init__(self, key: bytes = None):
        self.key = key or bytes([random.randint(1, 255) for _ in range(16)])
    
    def encode(self, data: bytes) -> bytes:
        """XOR encode data"""
        return bytes([b ^ self.key[i % len(self.key)] for i, b in enumerate(data)])
    
    def decode(self, data: bytes) -> bytes:
        """XOR decode data (same as encode for XOR)"""
        return self.encode(data)
    
    def get_key(self) -> bytes:
        """Get the XOR key"""
        return self.key


class Base64Encoder(Encoder):
    """Base64 encoding"""
    
    def encode(self, data: bytes) -> bytes:
        """Base64 encode data"""
        return base64.b64encode(data)
    
    def decode(self, data: bytes) -> bytes:
        """Base64 decode data"""
        return base64.b64decode(data)


class PolymorphicEncoder:
    """Polymorphic transformation engine"""
    
    def __init__(self):
        self.encoders: List[Encoder] = []
    
    def add_encoder(self, encoder: Encoder):
        """Add an encoder to the chain"""
        self.encoders.append(encoder)
        return self
    
    def transform(self, data: bytes) -> bytes:
        """Apply all encoders in sequence"""
        result = data
        for encoder in self.encoders:
            result = encoder.encode(result)
        return result
    
    def reverse_transform(self, data: bytes) -> bytes:
        """Reverse all encoders in reverse order"""
        result = data
        for encoder in reversed(self.encoders):
            result = encoder.decode(result)
        return result


class SignatureMutator:
    """Signature mutation for evasion"""
    
    @staticmethod
    def insert_nops(shellcode: bytes, nop_count: int = 10) -> bytes:
        """Insert NOP instructions randomly"""
        result = bytearray(shellcode)
        positions = sorted(random.sample(range(len(result)), min(nop_count, len(result))))
        
        for pos in reversed(positions):
            result.insert(pos, 0x90)  # NOP instruction
        
        return bytes(result)
    
    @staticmethod
    def add_junk_code(code: bytes, junk_ratio: float = 0.1) -> bytes:
        """Add junk code for obfuscation"""
        junk_size = int(len(code) * junk_ratio)
        junk = bytes([random.randint(0, 255) for _ in range(junk_size)])
        
        # Interleave junk with real code
        result = bytearray()
        chunk_size = max(1, len(code) // (junk_size + 1))
        
        for i in range(0, len(code), chunk_size):
            result.extend(code[i:i+chunk_size])
            if i + chunk_size < len(code) and len(junk) > 0:
                result.append(junk[0])
                junk = junk[1:]
        
        return bytes(result)


def create_encoder_chain(*encoder_types: str) -> PolymorphicEncoder:
    """Create a chain of encoders"""
    chain = PolymorphicEncoder()
    
    for etype in encoder_types:
        if etype == 'xor':
            chain.add_encoder(XorEncoder())
        elif etype == 'base64':
            chain.add_encoder(Base64Encoder())
        else:
            raise ValueError(f"Unknown encoder type: {etype}")
    
    return chain
