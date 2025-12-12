"""
RedNet Payload Generator CLI
Command-line interface for payload generation
"""

import click
from .generator import PayloadGenerator, PayloadType, create_payload
from .encoders import create_encoder_chain, XorEncoder
from .evasion import EvasionTechnique, SandboxDetector
from .automation import AutomationFramework


@click.group()
@click.version_option(version='1.0.0')
def cli():
    """RedNet Payload Generator - Security Testing Tool"""
    pass


@cli.command()
@click.option('--type', '-t', type=click.Choice(['benign_test', 'shellcode', 'script', 'binary']),
              default='benign_test', help='Payload type')
@click.option('--output', '-o', required=True, help='Output file path')
@click.option('--message', '-m', help='Custom message for benign test')
@click.option('--size', '-s', type=int, default=256, help='Payload size')
def generate(type, output, message, size):
    """Generate a test payload"""
    click.echo(f"🛠️  Generating {type} payload...")
    
    config = {'size': size}
    if message:
        config['message'] = message
    
    payload = create_payload(type, **config)
    
    with open(output, 'wb') as f:
        f.write(payload)
    
    click.echo(f"✅ Payload generated: {output} ({len(payload)} bytes)")


@cli.command()
@click.argument('input_file')
@click.option('--output', '-o', required=True, help='Output file')
@click.option('--encoders', '-e', multiple=True, help='Encoder chain (xor, base64)')
def encode(input_file, output, encoders):
    """Encode a payload"""
    click.echo(f"🔐 Encoding {input_file}...")
    
    with open(input_file, 'rb') as f:
        data = f.read()
    
    if not encoders:
        encoders = ['xor', 'base64']
    
    chain = create_encoder_chain(*encoders)
    encoded = chain.transform(data)
    
    with open(output, 'wb') as f:
        f.write(encoded)
    
    click.echo(f"✅ Encoded payload: {output} ({len(encoded)} bytes)")
    click.echo(f"   Encoders used: {', '.join(encoders)}")


@cli.command()
def detect():
    """Run sandbox detection checks"""
    click.echo("🔍 Running sandbox detection...")
    
    detector = SandboxDetector()
    
    click.echo("\n📊 Detection Results:")
    click.echo(f"  VM Detected: {detector.detect_vm()}")
    click.echo(f"  Debugger Detected: {detector.detect_debugger()}")
    click.echo(f"  Internet Available: {detector.check_internet()}")
    click.echo(f"  Disk Size: {detector.check_disk_size()} GB")
    click.echo(f"  CPU Count: {detector.check_cpu_count()}")


@cli.command()
@click.argument('target')
@click.option('--scan-type', '-s', default='basic', help='Scan type')
def automate(target, scan_type):
    """Run automation framework"""
    click.echo(f"🤖 Running automation on {target}...")
    
    framework = AutomationFramework()
    result = framework.run_scan(target, scan_type)
    
    click.echo("\n📊 Scan Results:")
    click.echo(f"  Target: {result['target']}")
    click.echo(f"  Status: {result['status']}")
    click.echo(f"  Findings: {len(result['findings'])}")
    
    for finding in result['findings']:
        click.echo(f"    - Port {finding['port']}: {finding['service']} ({finding['state']})")


if __name__ == '__main__':
    cli()
