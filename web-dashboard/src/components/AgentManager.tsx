'use client'

import { useState } from 'react'

interface Agent {
    id: string
    name: string
    status: 'online' | 'offline' | 'warning'
    ip: string
    lastSeen: string
    packets: number
}

const mockAgents: Agent[] = [
    { id: '1', name: 'Agent-Alpha', status: 'online', ip: '192.168.1.10', lastSeen: '2s ago', packets: 15420 },
    { id: '2', name: 'Agent-Beta', status: 'online', ip: '192.168.1.11', lastSeen: '5s ago', packets: 8932 },
    { id: '3', name: 'Agent-Gamma', status: 'warning', ip: '192.168.1.12', lastSeen: '45s ago', packets: 3201 },
    { id: '4', name: 'Agent-Delta', status: 'offline', ip: '192.168.1.13', lastSeen: '5m ago', packets: 0 },
]

export default function AgentManager() {
    const [agents] = useState<Agent[]>(mockAgents)

    const getStatusColor = (status: Agent['status']) => {
        switch (status) {
            case 'online':
                return 'bg-green-500'
            case 'warning':
                return 'bg-yellow-500'
            case 'offline':
                return 'bg-red-500'
        }
    }

    const getStatusText = (status: Agent['status']) => {
        switch (status) {
            case 'online':
                return 'text-green-400'
            case 'warning':
                return 'text-yellow-400'
            case 'offline':
                return 'text-red-400'
        }
    }

    return (
        <div className="space-y-4">
            <div className="flex items-center justify-between">
                <h2 className="text-2xl font-bold text-white">Agent Management</h2>
                <button className="px-4 py-2 glass cyber-border rounded-lg glass-hover text-neon-blue font-medium">
                    + Add Agent
                </button>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
                {agents.map((agent) => (
                    <div
                        key={agent.id}
                        className="glass cyber-border rounded-xl p-5 glass-hover scan-line"
                    >
                        <div className="flex items-start justify-between mb-4">
                            <div className="flex items-center gap-2">
                                <div className={`w-3 h-3 rounded-full ${getStatusColor(agent.status)} animate-pulse`} />
                                <span className={`text-sm font-semibold ${getStatusText(agent.status)}`}>
                                    {agent.status.toUpperCase()}
                                </span>
                            </div>
                            <button className="text-gray-400 hover:text-white transition-colors">
                                ⚙️
                            </button>
                        </div>

                        <h3 className="text-lg font-bold text-white mb-2">{agent.name}</h3>

                        <div className="space-y-2 text-sm">
                            <div className="flex justify-between">
                                <span className="text-gray-400">IP Address</span>
                                <span className="text-gray-200 font-mono">{agent.ip}</span>
                            </div>
                            <div className="flex justify-between">
                                <span className="text-gray-400">Last Seen</span>
                                <span className="text-gray-200">{agent.lastSeen}</span>
                            </div>
                            <div className="flex justify-between">
                                <span className="text-gray-400">Packets</span>
                                <span className="text-neon-blue font-semibold">{agent.packets.toLocaleString()}</span>
                            </div>
                        </div>

                        <div className="mt-4 pt-4 border-t border-white/10 flex gap-2">
                            <button className="flex-1 px-3 py-2 bg-neon-blue/20 hover:bg-neon-blue/30 border border-neon-blue/50 rounded-lg text-xs font-medium text-neon-blue transition-all">
                                View Logs
                            </button>
                            <button className="flex-1 px-3 py-2 bg-neon-purple/20 hover:bg-neon-purple/30 border border-neon-purple/50 rounded-lg text-xs font-medium text-neon-purple transition-all">
                                Commands
                            </button>
                        </div>
                    </div>
                ))}
            </div>
        </div>
    )
}
