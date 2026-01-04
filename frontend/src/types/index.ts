export interface User {
  id: string
  email: string
  username: string
  created_at: string
  updated_at: string
}

export interface Message {
  id: string
  user_id: string
  content: string
  created_at: string
  updated_at: string
}

export interface LoginRequest {
  email: string
  password: string
}

export interface LoginResponse {
  token: string
  user: User
}

export type Theme = 'auto' | 'light' | 'dark'

export type PendingOpType = 'create' | 'update' | 'delete'

export interface PendingOp {
  id: string
  type: PendingOpType
  data: any
  timestamp: string
  retries: number
}
