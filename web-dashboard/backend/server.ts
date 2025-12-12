import express from 'express'
import { createServer } from 'http'
import { Server } from 'socket.io'
import cors from 'cors'
import jwt from 'jsonwebtoken'
import bcrypt from 'bcryptjs'

const app = express()
const httpServer = createServer(app)
const io = new Server(httpServer, {
    cors: {
        origin: 'http://localhost:3000',
        methods: ['GET', 'POST'],
    },
})

const PORT = process.env.PORT || 3001
const JWT_SECRET = process.env.JWT_SECRET || 'rednet-secret-key-change-in-production'

app.use(cors())
app.use(express.json())

// Mock database
const users = [
    {
        id: '1',
        username: 'admin',
        password: bcrypt.hashSync('admin123', 10),
        role: 'admin',
    },
]

// Authentication endpoint
app.post('/api/auth/login', async (req, res) => {
    const { username, password } = req.body

    const user = users.find(u => u.username === username)
    if (!user) {
        return res.status(401).json({ error: 'Invalid credentials' })
    }

    const validPassword = await bcrypt.compare(password, user.password)
    if (!validPassword) {
        return res.status(401).json({ error: 'Invalid credentials' })
    }

    const token = jwt.sign(
        { id: user.id, username: user.username, role: user.role },
        JWT_SECRET,
        { expiresIn: '24h' }
    )

    res.json({ token, user: { id: user.id, username: user.username, role: user.role } })
})

// Stats endpoint
app.get('/api/stats', (req, res) => {
    res.json({
        totalPackets: Math.floor(Math.random() * 100000),
        activeAgents: 4,
        threatsDetected: Math.floor(Math.random() * 10),
        bandwidth: Math.floor(Math.random() * 150),
    })
})

// WebSocket connection handling
io.on('connection', (socket) => {
    console.log('Client connected:', socket.id)

    // Send initial stats
    socket.emit('stats', {
        totalPackets: 45230,
        activeAgents: 4,
        threatsDetected: 3,
        bandwidth: 85,
    })

    // Simulate real-time updates
    const interval = setInterval(() => {
        socket.emit('stats', {
            totalPackets: Math.floor(Math.random() * 100000),
            activeAgents: Math.floor(Math.random() * 10),
            threatsDetected: Math.floor(Math.random() * 20),
            bandwidth: Math.floor(Math.random() * 200),
        })
    }, 3000)

    socket.on('disconnect', () => {
        console.log('Client disconnected:', socket.id)
        clearInterval(interval)
    })

    socket.on('command', (data) => {
        console.log('Received command:', data)
        socket.emit('command-response', {
            success: true,
            message: 'Command executed successfully',
        })
    })
})

httpServer.listen(PORT, () => {
    console.log(`🚀 RedNet Backend Server running on port ${PORT}`)
    console.log(`📡 WebSocket server ready`)
    console.log(`🔐 API endpoints available at http://localhost:${PORT}/api`)
})
