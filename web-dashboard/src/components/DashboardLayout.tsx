'use client'

import { ReactNode } from 'react'
import Link from 'next/link'
import { usePathname } from 'next/navigation'

export default function DashboardLayout({ children }: { children: ReactNode }) {
    const pathname = usePathname()

    const navItems = [
        { name: 'Dashboard', path: '/', icon: '📊' },
        { name: 'Packet Capture', path: '/capture', icon: '📡' },
        { name: 'Encryption', path: '/encryption', icon: '🔐' },
        { name: 'Payloads', path: '/payloads', icon: '🛠️' },
        { name: 'Agents', path: '/agents', icon: '🤖' },
        { name: 'Logs', path: '/logs', icon: '📝' },
    ]

    return (
        <div className="min-h-screen flex">
            {/* Sidebar */}
            <aside className="w-64 glass border-r border-white/10 p-6">
                <div className="mb-8">
                    <h2 className="text-2xl font-bold neon-text mb-1">RedNet</h2>
                    <p className="text-xs text-gray-500">Security Toolkit v1.0</p>
                </div>

                <nav className="space-y-2">
                    {navItems.map((item) => {
                        const isActive = pathname === item.path
                        return (
                            <Link
                                key={item.path}
                                href={item.path}
                                className={`flex items-center gap-3 px-4 py-3 rounded-lg transition-all duration-200 ${isActive
                                        ? 'bg-neon-blue/20 border border-neon-blue/50 text-neon-blue'
                                        : 'glass-hover text-gray-400'
                                    }`}
                            >
                                <span className="text-xl">{item.icon}</span>
                                <span className="font-medium">{item.name}</span>
                            </Link>
                        )
                    })}
                </nav>

                <div className="mt-auto pt-8">
                    <div className="glass cyber-border rounded-lg p-4">
                        <h3 className="text-sm font-semibold mb-2">System Status</h3>
                        <div className="space-y-2 text-xs">
                            <div className="flex justify-between">
                                <span className="text-gray-400">CPU</span>
                                <span className="text-green-400">23%</span>
                            </div>
                            <div className="flex justify-between">
                                <span className="text-gray-400">Memory</span>
                                <span className="text-yellow-400">67%</span>
                            </div>
                            <div className="flex justify-between">
                                <span className="text-gray-400">Network</span>
                                <span className="text-blue-400">Active</span>
                            </div>
                        </div>
                    </div>
                </div>
            </aside>

            {/* Main Content */}
            <main className="flex-1 p-8 overflow-y-auto">
                <div className="max-w-7xl mx-auto">
                    {children}
                </div>
            </main>
        </div>
    )
}
