'use client'

import { LineChart, Line, AreaChart, Area, PieChart, Pie, Cell, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer } from 'recharts'

const trafficData = [
    { time: '00:00', packets: 1200, bandwidth: 45 },
    { time: '04:00', packets: 800, bandwidth: 30 },
    { time: '08:00', packets: 2400, bandwidth: 85 },
    { time: '12:00', packets: 3200, bandwidth: 120 },
    { time: '16:00', packets: 2800, bandwidth: 95 },
    { time: '20:00', packets: 1600, bandwidth: 60 },
]

const protocolData = [
    { name: 'HTTP', value: 35, color: '#00d9ff' },
    { name: 'HTTPS', value: 45, color: '#bd00ff' },
    { name: 'DNS', value: 10, color: '#00ff9f' },
    { name: 'TCP', value: 8, color: '#ff006e' },
    { name: 'Other', value: 2, color: '#ffd700' },
]

export default function NetworkTelemetry() {
    return (
        <div className="space-y-6">
            <h2 className="text-2xl font-bold text-white mb-4">Network Telemetry</h2>

            <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                {/* Traffic Chart */}
                <div className="glass cyber-border rounded-xl p-6">
                    <h3 className="text-lg font-semibold mb-4 text-neon-blue">Traffic Over Time</h3>
                    <ResponsiveContainer width="100%" height={300}>
                        <AreaChart data={trafficData}>
                            <defs>
                                <linearGradient id="colorPackets" x1="0" y1="0" x2="0" y2="1">
                                    <stop offset="5%" stopColor="#00d9ff" stopOpacity={0.8} />
                                    <stop offset="95%" stopColor="#00d9ff" stopOpacity={0} />
                                </linearGradient>
                            </defs>
                            <CartesianGrid strokeDasharray="3 3" stroke="#ffffff10" />
                            <XAxis dataKey="time" stroke="#888" />
                            <YAxis stroke="#888" />
                            <Tooltip
                                contentStyle={{
                                    backgroundColor: 'rgba(0, 0, 0, 0.8)',
                                    border: '1px solid rgba(0, 217, 255, 0.3)',
                                    borderRadius: '8px',
                                }}
                            />
                            <Area
                                type="monotone"
                                dataKey="packets"
                                stroke="#00d9ff"
                                fillOpacity={1}
                                fill="url(#colorPackets)"
                            />
                        </AreaChart>
                    </ResponsiveContainer>
                </div>

                {/* Protocol Distribution */}
                <div className="glass cyber-border rounded-xl p-6">
                    <h3 className="text-lg font-semibold mb-4 text-neon-purple">Protocol Distribution</h3>
                    <ResponsiveContainer width="100%" height={300}>
                        <PieChart>
                            <Pie
                                data={protocolData}
                                cx="50%"
                                cy="50%"
                                labelLine={false}
                                label={({ name, percent }) => `${name} ${(percent * 100).toFixed(0)}%`}
                                outerRadius={100}
                                fill="#8884d8"
                                dataKey="value"
                            >
                                {protocolData.map((entry, index) => (
                                    <Cell key={`cell-${index}`} fill={entry.color} />
                                ))}
                            </Pie>
                            <Tooltip
                                contentStyle={{
                                    backgroundColor: 'rgba(0, 0, 0, 0.8)',
                                    border: '1px solid rgba(189, 0, 255, 0.3)',
                                    borderRadius: '8px',
                                }}
                            />
                        </PieChart>
                    </ResponsiveContainer>
                </div>

                {/* Bandwidth Chart */}
                <div className="glass cyber-border rounded-xl p-6 lg:col-span-2">
                    <h3 className="text-lg font-semibold mb-4 text-neon-green">Bandwidth Utilization</h3>
                    <ResponsiveContainer width="100%" height={250}>
                        <LineChart data={trafficData}>
                            <CartesianGrid strokeDasharray="3 3" stroke="#ffffff10" />
                            <XAxis dataKey="time" stroke="#888" />
                            <YAxis stroke="#888" />
                            <Tooltip
                                contentStyle={{
                                    backgroundColor: 'rgba(0, 0, 0, 0.8)',
                                    border: '1px solid rgba(0, 255, 159, 0.3)',
                                    borderRadius: '8px',
                                }}
                            />
                            <Legend />
                            <Line
                                type="monotone"
                                dataKey="bandwidth"
                                stroke="#00ff9f"
                                strokeWidth={2}
                                dot={{ fill: '#00ff9f', r: 4 }}
                                activeDot={{ r: 6 }}
                            />
                        </LineChart>
                    </ResponsiveContainer>
                </div>
            </div>
        </div>
    )
}
