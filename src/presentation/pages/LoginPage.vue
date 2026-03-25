<script setup lang="ts">
import { ref } from "vue";
import { useRouter } from "vue-router";
import { useAuthStore } from "../stores";

const router = useRouter();
const authStore = useAuthStore();

const username = ref("");
const password = ref("");
const isLoading = ref(false);

async function handleLogin() {
  if (!username.value || !password.value) return;
  
  isLoading.value = true;
  const success = await authStore.login(username.value, password.value);
  isLoading.value = false;
  
  if (success) {
    router.push({ name: "home" });
  }
}
</script>

<template>
  <div class="login-container">
    <div class="login-card">
      <h1>Iniciar Sesión</h1>
      
      <form @submit.prevent="handleLogin">
        <div class="form-group">
          <label for="username">Usuario</label>
          <input
            id="username"
            v-model="username"
            type="text"
            placeholder="Ingrese su usuario"
            :disabled="isLoading"
          />
        </div>
        
        <div class="form-group">
          <label for="password">Contraseña</label>
          <input
            id="password"
            v-model="password"
            type="password"
            placeholder="Ingrese su contraseña"
            :disabled="isLoading"
          />
        </div>
        
        <div v-if="authStore.error" class="error-message">
          {{ authStore.error }}
        </div>
        
        <button type="submit" :disabled="isLoading || !username || !password">
          {{ isLoading ? "Iniciando sesión..." : "Entrar" }}
        </button>
      </form>
    </div>
  </div>
</template>

<style scoped>
.login-container {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.login-card {
  background: white;
  padding: 2rem;
  border-radius: 12px;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.2);
  width: 100%;
  max-width: 400px;
}

h1 {
  margin: 0 0 1.5rem;
  text-align: center;
  color: #333;
}

.form-group {
  margin-bottom: 1rem;
}

label {
  display: block;
  margin-bottom: 0.5rem;
  color: #555;
  font-weight: 500;
}

input {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid #ddd;
  border-radius: 6px;
  font-size: 1rem;
  box-sizing: border-box;
}

input:focus {
  outline: none;
  border-color: #667eea;
}

button {
  width: 100%;
  padding: 0.75rem;
  background: #667eea;
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 1rem;
  font-weight: 500;
  cursor: pointer;
  margin-top: 0.5rem;
}

button:hover:not(:disabled) {
  background: #5568d3;
}

button:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.error-message {
  color: #e53e3e;
  margin-bottom: 1rem;
  padding: 0.75rem;
  background: #fed7d7;
  border-radius: 6px;
  text-align: center;
}
</style>
