<script setup lang="ts">
import { ref } from 'vue'
import { type ChatInfo } from '../service/chat'

const emit = defineEmits<{
  (e: 'connect', chatInfo: ChatInfo): void
  (e: 'disconnect'): void
}>()

const props = defineProps<{
  is_logged: boolean
  error_message?: string
}>()

const chatInfo = ref<ChatInfo>({ username: '', chat: '' })
</script>

<template>
  <div class="board">
    <div class="row">
      <div class="row_label">Username</div>
      <div class="row_input">
        <input type="text" :disabled="is_logged" v-model="chatInfo.username" />
      </div>
    </div>
    <div class="row">
      <div class="row_label">Room name</div>
      <div class="row_input">
        <input type="text" :disabled="is_logged" v-model="chatInfo.chat" />
      </div>
    </div>
    <div class="row buttons">
      <div class="button connect_button">
        <button :disabled="is_logged" @click="$emit('connect', chatInfo)">Connect</button>
      </div>
      <div class="button disconnect_button" @click="$emit('disconnect')">
        <button :disabled="!is_logged">Disconnect</button>
      </div>
    </div>
    <div class="row error">
      {{ error_message ? error_message : '' }}
    </div>
  </div>
</template>

<style scoped>
.row {
  margin-bottom: 10px;
}

.board {
  padding: 20px 20px 10px 20px;
  display: inline-block;
}

.buttons {
  display: flex;
  justify-content: space-evenly;
}

.row_input input {
  padding: 10px 15px;
  width: 300px;
  outline: none;
  border: none;
  border-radius: 5px;
  box-shadow: 0px 2px 2px black;
}

.buttons button {
  padding: 10px 20px;
  margin-top: 20px;
  border-radius: 5px;
  box-shadow: 0px 2px 2px black;
  width: 120px;
  border: none;
  transition: 0.1s;
}

.connect_button button {
  background: #e6f3e6;
}

.disconnect_button button {
  background: #f3e6f2;
}

.connect_button button:active {
  background: #b8e3b8;
}

.disconnect_button button:active {
  background: #e3b8d9;
}

.error {
  color: red;
  margin-bottom: 0;
  text-align: center;
}
</style>
