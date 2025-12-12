"""
Evasion Techniques Module
Sandbox detection and evasion simulations
"""

import time
import platform
import os
from typing import Dict, List


class EvasionTechnique:
    """Base class for evasion techniques"""
    
    @staticmethod
    def sleep_obfuscation(duration: float = 5.0):
        """Sleep to evade time-based sandboxes"""
        print(f"[EVASION] Sleeping for {duration} seconds...")
        time.sleep(duration)
    
    @staticmethod
    def check_execution_count(threshold: int = 3) -> bool:
        """Simulate execution count check"""
        # In real scenario, would check registry/file
        print(f"[EVASION] Checking execution count (threshold: {threshold})")
        return True
    
    @staticmethod
    def environment_checks() -> Dict[str, any]:
        """Perform environment checks"""
        checks = {
            'is_vm': SandboxDetector.detect_vm(),
            'is_debugger': SandboxDetector.detect_debugger(),
            'has_internet': SandboxDetector.check_internet(),
            'system_info': SandboxDetector.get_system_info(),
        }
        return checks


class SandboxDetector:
    """Sandbox and VM detection techniques"""
    
    @staticmethod
    def detect_vm() -> bool:
        """Detect if running in a VM (simulation)"""
        print("[DETECTION] Checking for VM indicators...")
        
        # Check for common VM artifacts
        vm_indicators = [
            'vmware', 'virtualbox', 'qemu', 'xen', 'hyper-v'
        ]
        
        system_info = platform.platform().lower()
        for indicator in vm_indicators:
            if indicator in system_info:
                print(f"[DETECTION] VM detected: {indicator}")
                return True
        
        print("[DETECTION] No VM detected")
        return False
    
    @staticmethod
    def detect_debugger() -> bool:
        """Detect if debugger is attached (simulation)"""
        print("[DETECTION] Checking for debugger...")
        
        # In real scenario, would check actual debugger presence
        # This is just a simulation
        if os.environ.get('REDNET_DEBUG'):
            print("[DETECTION] Debugger detected")
            return True
        
        print("[DETECTION] No debugger detected")
        return False
    
    @staticmethod
    def check_internet() -> bool:
        """Check for internet connectivity"""
        print("[DETECTION] Checking internet connectivity...")
        
        try:
            import socket
            socket.create_connection(("8.8.8.8", 53), timeout=2)
            print("[DETECTION] Internet connection available")
            return True
        except OSError:
            print("[DETECTION] No internet connection")
            return False
    
    @staticmethod
    def get_system_info() -> Dict[str, str]:
        """Get system information"""
        return {
            'platform': platform.system(),
            'release': platform.release(),
            'version': platform.version(),
            'machine': platform.machine(),
            'processor': platform.processor(),
        }
    
    @staticmethod
    def check_disk_size() -> int:
        """Check disk size (VMs often have small disks)"""
        print("[DETECTION] Checking disk size...")
        
        try:
            import shutil
            total, used, free = shutil.disk_usage("/")
            total_gb = total // (2**30)
            print(f"[DETECTION] Total disk size: {total_gb} GB")
            return total_gb
        except Exception as e:
            print(f"[DETECTION] Error checking disk size: {e}")
            return 0
    
    @staticmethod
    def check_cpu_count() -> int:
        """Check CPU count (VMs often have few CPUs)"""
        import multiprocessing
        cpu_count = multiprocessing.cpu_count()
        print(f"[DETECTION] CPU count: {cpu_count}")
        return cpu_count


class AntiAnalysis:
    """Anti-analysis techniques"""
    
    @staticmethod
    def check_analysis_tools() -> List[str]:
        """Check for common analysis tools"""
        print("[ANTI-ANALYSIS] Checking for analysis tools...")
        
        analysis_tools = [
            'wireshark', 'tcpdump', 'procmon', 'processhacker',
            'ida', 'ollydbg', 'x64dbg', 'ghidra'
        ]
        
        detected = []
        # In real scenario, would check running processes
        print("[ANTI-ANALYSIS] No analysis tools detected (simulation)")
        return detected
    
    @staticmethod
    def timing_check(expected_duration: float = 0.1) -> bool:
        """Check for timing anomalies"""
        print("[ANTI-ANALYSIS] Performing timing check...")
        
        start = time.time()
        # Perform some computation
        _ = sum(range(100000))
        elapsed = time.time() - start
        
        if elapsed > expected_duration * 10:
            print(f"[ANTI-ANALYSIS] Timing anomaly detected: {elapsed:.4f}s")
            return True
        
        print(f"[ANTI-ANALYSIS] Timing normal: {elapsed:.4f}s")
        return False
