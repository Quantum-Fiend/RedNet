'use client'

import { useEffect, useState, useCallback } from 'react'
import { io, Socket } from 'socket.io-client'

export function useWebSocket(url: string) {
    const [socket, setSocket] = useState<Socket | null>(null)
    const [connected, setConnected] = useState(false)
    const [data, setData] = useState<any>(null)

    useEffect(() => {
        const socketInstance = io(url, {
            transports: ['websocket'],
            reconnection: true,
            reconnectionDelay: 1000,
            reconnectionAttempts: 5,
        })

        socketInstance.on('connect', () => {
            console.log('WebSocket connected')
            setConnected(true)
        })

        socketInstance.on('disconnect', () => {
            console.log('WebSocket disconnected')
            setConnected(false)
        })

        socketInstance.on('data', (receivedData: any) => {
            setData(receivedData)
        })

        socketInstance.on('stats', (stats: any) => {
            setData(stats)
        })

        setSocket(socketInstance)

        return () => {
            socketInstance.disconnect()
        }
    }, [url])

    const sendMessage = useCallback((event: string, message: any) => {
        if (socket && connected) {
            socket.emit(event, message)
        }
    }, [socket, connected])

    return { socket, connected, data, sendMessage }
}
