'use client'

import { useState, useEffect } from 'react'
import DashboardLayout from '@/components/DashboardLayout'
import NetworkTelemetry from '@/components/NetworkTelemetry'
import AgentManager from '@/components/AgentManager'
import { useWebSocket } from '@/hooks/useWebSocket'

export default function Home() {
    const { connected, data, sendMessage } = useWebSocket('ws://localhost:3001')
    const [stats, setStats] = useState({
        totalPackets: 0,
        activeAgents: 0,
        threatsDetected: 0,
        bandwidth: 0,
    })

    useEffect(() => {
        if (data) {
            // Update stats from WebSocket data
            setStats(prev => ({
                ...prev,
                ...data,
            }))
        }
    }, [data])

    return (
        <DashboardLayout>
            <div className="space-y-6">
                {/* Header */}
                <div className="flex items-center justify-between">
                    <div>
                        <h1 className="text-4xl font-bold neon-text mb-2">
                            RedNet Dashboard
                        </h1>
                        <p className="text-gray-400">
                            Multi-Language Cybersecurity Toolkit
                        </p>
                    </div>
                    <div className="flex items-center gap-3">
                        <div className={`flex items-center gap-2 px-4 py-2 rounded-lg glass ${connected ? 'border-green-500/50' : 'border-red-500/50'}`}>
                            <div className={`w-2 h-2 rounded-full ${connected ? 'bg-green-500 animate-pulse' : 'bg-red-500'}`} />
                            <span className="text-sm">{connected ? 'Connected' : 'Disconnected'}</span>
                        </div>
                    </div>
                </div>

                {/* Stats Grid */}
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
                    <StatCard
                        title="Total Packets"
                        value={stats.totalPackets.toLocaleString()}
                        icon="📦"
                        trend="+12.5%"
                    />
                    <StatCard
                        title="Active Agents"
                        value={stats.activeAgents}
                        icon="🤖"
                        trend="+2"
                    />
                    <StatCard
                        title="Threats Detected"
                        value={stats.threatsDetected}
                        icon="⚠️"
                        trend="-5.2%"
                        trendPositive={false}
                    />
                    <StatCard
                        title="Bandwidth"
                        value={`${stats.bandwidth} Mbps`}
                        icon="📊"
                        trend="+8.3%"
                    />
                </div>

                {/* Network Telemetry */}
                <NetworkTelemetry />

                {/* Agent Manager */}
                <AgentManager />
            </div>
        </DashboardLayout>
    )
}

function StatCard({ title, value, icon, trend, trendPositive = true }: {
    title: string
    value: string | number
    icon: string
    trend: string
    trendPositive?: boolean
}) {
    return (
        <div className="glass cyber-border rounded-xl p-6 glass-hover">
            <div className="flex items-start justify-between mb-4">
                <span className="text-3xl">{icon}</span>
                <span className={`text-sm px-2 py-1 rounded ${trendPositive ? 'bg-green-500/20 text-green-400' : 'bg-red-500/20 text-red-400'}`}>
                    {trend}
                </span>
            </div>
            <h3 className="text-gray-400 text-sm mb-1">{title}</h3>
            <p className="text-2xl font-bold text-white">{value}</p>
        </div>
    )
}
