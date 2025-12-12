"""
RedNet Payload Generator
Benign test payload creation and red-team automation framework
"""

__version__ = "1.0.0"
__author__ = "RedNet Security Team"

from .generator import PayloadGenerator, PayloadType
from .encoders import Encoder, XorEncoder, Base64Encoder
from .evasion import EvasionTechnique, SandboxDetector
from .automation import AutomationFramework

__all__ = [
    'PayloadGenerator',
    'PayloadType',
    'Encoder',
    'XorEncoder',
    'Base64Encoder',
    'EvasionTechnique',
    'SandboxDetector',
    'AutomationFramework',
]
