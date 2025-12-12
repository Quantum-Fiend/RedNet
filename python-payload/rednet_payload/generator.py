"""
Payload Generator Module
Creates benign test payloads for security testing
"""

from enum import Enum
from typing import Dict, Any, Optional
import os
import struct


class PayloadType(Enum):
    """Supported payload types"""
    BENIGN_TEST = "benign_test"
    SHELLCODE = "shellcode"
    SCRIPT = "script"
    BINARY = "binary"


class PayloadGenerator:
    """Main payload generation class"""
    
    def __init__(self, payload_type: PayloadType = PayloadType.BENIGN_TEST):
        self.payload_type = payload_type
        self.config: Dict[str, Any] = {}
    
    def configure(self, **kwargs):
        """Configure payload generation parameters"""
        self.config.update(kwargs)
        return self
    
    def generate(self, output_path: Optional[str] = None) -> bytes:
        """Generate the payload"""
        if self.payload_type == PayloadType.BENIGN_TEST:
            payload = self._generate_benign_test()
        elif self.payload_type == PayloadType.SHELLCODE:
            payload = self._generate_shellcode()
        elif self.payload_type == PayloadType.SCRIPT:
            payload = self._generate_script()
        elif self.payload_type == PayloadType.BINARY:
            payload = self._generate_binary()
        else:
            raise ValueError(f"Unsupported payload type: {self.payload_type}")
        
        if output_path:
            with open(output_path, 'wb') as f:
                f.write(payload)
        
        return payload
    
    def _generate_benign_test(self) -> bytes:
        """Generate a benign test payload"""
        message = self.config.get('message', 'RedNet Test Payload - Benign')
        return message.encode('utf-8')
    
    def _generate_shellcode(self) -> bytes:
        """Generate test shellcode (NOP sled + marker)"""
        size = self.config.get('size', 256)
        
        # NOP sled (0x90)
        nops = bytes([0x90] * (size - 16))
        
        # Marker
        marker = b"REDNET_SHELLCODE"
        
        return nops + marker
    
    def _generate_script(self) -> bytes:
        """Generate a test script payload"""
        script_type = self.config.get('script_type', 'python')
        
        if script_type == 'python':
            script = '''#!/usr/bin/env python3
# RedNet Test Script
import sys

def main():
    print("RedNet Security Test - Benign Payload")
    print("This is a harmless test script")
    return 0

if __name__ == "__main__":
    sys.exit(main())
'''
        elif script_type == 'bash':
            script = '''#!/bin/bash
# RedNet Test Script
echo "RedNet Security Test - Benign Payload"
echo "This is a harmless test script"
exit 0
'''
        elif script_type == 'powershell':
            script = '''# RedNet Test Script
Write-Host "RedNet Security Test - Benign Payload"
Write-Host "This is a harmless test script"
Exit 0
'''
        else:
            script = f"# Unknown script type: {script_type}\n"
        
        return script.encode('utf-8')
    
    def _generate_binary(self) -> bytes:
        """Generate a test binary payload"""
        # Simple PE/ELF header simulation (not a real executable)
        header = b"MZ\x90\x00"  # DOS header magic
        padding = bytes([0x00] * 60)
        marker = b"REDNET_TEST_BINARY"
        
        return header + padding + marker


class PayloadTemplate:
    """Template system for payload creation"""
    
    @staticmethod
    def get_template(name: str) -> str:
        """Get a payload template by name"""
        templates = {
            'reverse_shell_sim': '''
# Simulated Reverse Shell (BENIGN - FOR TESTING ONLY)
import socket
import sys

def simulate_reverse_shell():
    print("[SIMULATION] This would connect to a remote host")
    print("[SIMULATION] No actual connection is made")
    print("[SIMULATION] Target: {host}:{port}")
    
if __name__ == "__main__":
    simulate_reverse_shell()
''',
            'file_exfil_sim': '''
# Simulated File Exfiltration (BENIGN - FOR TESTING ONLY)
import os

def simulate_exfiltration():
    print("[SIMULATION] This would exfiltrate files")
    print("[SIMULATION] No actual data is transmitted")
    print("[SIMULATION] Target directory: {target_dir}")
    
if __name__ == "__main__":
    simulate_exfiltration()
''',
        }
        
        return templates.get(name, "# Template not found")


def create_payload(payload_type: str, **config) -> bytes:
    """Convenience function to create a payload"""
    ptype = PayloadType(payload_type)
    generator = PayloadGenerator(ptype)
    generator.configure(**config)
    return generator.generate()
