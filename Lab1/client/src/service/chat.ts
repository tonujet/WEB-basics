const getUrl = ({ chat, username }: ChatInfo) => {
  return `ws://127.0.0.1:3030/chat/${chat}?username=${username}`
}

interface ChatInfo {
  chat: string
  username: string
}

interface IncomingMessage {
  text: string
}

interface ChatMessage {
  username: string
  text: string
}

interface ErrorMessage {
  error_message: string
}

const connectToChat = (chatInfo: ChatInfo): Promise<WebSocket> => {
  return new Promise((resolve, reject) => {
    const url = getUrl(chatInfo)
    const socket = new WebSocket(url)
    socket.onopen = () => {
      console.log(`WS connected to url: ${url}`)
      resolve(socket)
    }
    socket.onerror = (err) => {
      console.log(`IN wS connection to url: ${url} suddenly occured error: ${err}`)
      reject(err)
    }
  })
}

export { connectToChat }

export type { ChatInfo, ChatMessage, IncomingMessage, ErrorMessage }
