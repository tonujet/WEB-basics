<script setup lang="ts">
import { ref } from 'vue'
import type { ChatMessage } from '@/service/chat'

const input = ref('')

const props = defineProps<{
  messages: ChatMessage[]
  is_logged: boolean
}>()

const emit = defineEmits<(e: 'sendMessage', message: string) => void>()

const enterMessage = () => {
  if (!input.value.trim()) return
  emit('sendMessage', input.value)
  input.value = ''
}
</script>

<template>
  <div class="panel">
    <div class="messages">
      <div class="message" v-for="message in messages">
        <div class="username">@{{ message.username }}</div>
        <div class="text">
          {{ message.text }}
        </div>
      </div>
    </div>
    <div class="input">
      <div class="text">
        <input
          type="text"
          v-model="input"
          :disabled="!is_logged"
          @keyup.enter="enterMessage"
          placeholder="message"
        />
      </div>
    </div>
  </div>
</template>

<style scoped>
.panel {
  display: flex;
  height: 93%;
  flex-direction: column;
}

.messages {
  flex: 1 1 auto;
  overflow: auto;
}

.message {
  border: 1px solid black;
  margin: 4px;
  border-radius: 5px;
  padding: 10px 10px 10px 10px;
}

.username {
  margin-bottom: 10px;
  color: red;
}

.input input {
  padding: 10px 5px;
  width: 100%;
  box-shadow: 0px 1px 2px black;
}

.input {
  padding: 20px;
}
</style>
