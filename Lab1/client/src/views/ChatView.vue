<script setup lang="ts">
import LoginComponent from '@/components/LoginComponent.vue'
import ChatComponent from '@/components/ChatComponent.vue'
import { ref } from 'vue'
import {
  type ChatInfo,
  type ChatMessage,
  connectToChat,
  type ErrorMessage,
  type IncomingMessage
} from '@/service/chat'

const isLogged = ref(false)
const messages = ref<ChatMessage[]>([])
const chatInfo = ref<ChatInfo>({ chat: '', username: '' })
const errorMessage = ref('')
let error = ref<Error | null>(null)
let socket: null | WebSocket = null

const clear = () => {
  isLogged.value = false
  chatInfo.value = {} as ChatInfo
  messages.value = []
}

const connect = async (info: ChatInfo) => {
  try {
    errorMessage.value = ''
    if (!info.username.trim() || !info.chat.trim()) {
      return
    }
    socket = await connectToChat(info)
    socket.onmessage = (event) => {
      let requestedMessaged: ChatMessage[] | ErrorMessage = JSON.parse(event.data)
      if ('error_message' in requestedMessaged) {
        errorMessage.value = requestedMessaged['error_message']
        socket?.close()
        clear()
        return
      }
      console.log(requestedMessaged)
      messages.value.unshift(...requestedMessaged)
    }
    if (errorMessage.value) {
      return
    }
    isLogged.value = true
    chatInfo.value = info
  } catch (e: unknown) {
    if (e instanceof Error) error.value = e
    clear()
  }
}

const send_message = (text: string) => {
  const message: IncomingMessage = { text }

  if (socket) {
    socket.send(JSON.stringify(message))
    console.log(socket)
    messages.value.unshift({ username: chatInfo.value.username, text })
  }
}

const disconnect = () => {
  if (socket) {
    socket.close()
    isLogged.value = false
    messages.value = []
  }
}
</script>

<template>
  <div class="wrapper">
    <div class="login">
      <LoginComponent
        :is_logged="isLogged"
        :error_message="errorMessage"
        @connect="connect"
        @disconnect="disconnect"
      />
    </div>
    <ChatComponent :is_logged="isLogged" :messages="messages" @send-message="send_message" />
  </div>
</template>

<style scoped>
.login {
  background: #c5d7c5;
  display: flex;
  justify-content: center;
}

.wrapper {
  height: 80%;
}
</style>
