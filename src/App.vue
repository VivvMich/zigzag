<script setup lang="ts">
import {onMounted, ref} from "vue";
import {invoke} from "@tauri-apps/api/core";

const rows = ref([]);
const words = ref<string[]>([])

function generateTable() {
  const width = 31
  const height = 29
  const table = []

  for (let y = 0; y < height; y++) {
    const row = []
    for (let x = 0; x < width; x++) {
      row.push("#")
    }

    table.push(row)
  }

  rows.value = table
}

async function getWords(): Promise<string[]> {
   let words: string[]  = await invoke("get_words");
   return words;
}

onMounted( async() => {
  getWords();
  generateTable();
  words.value = await getWords();
})

</script>

<template>
  <main class="container">
    <h1>Générateur de mot en vrac</h1>
    <div class="word-list">
      <ul>
       <li v-for="word in words" :key="word">{{ word }}</li>
      </ul>
    </div>

    <!-- 31 x 29 => trou de 9 x 11 =>   -->
    <table id="grid-word">
      <tbody>
      <tr v-for="(row, i) in rows" :key="i">
        <td v-for="(cell, j) in row" :key="j">{{ cell }}</td>
      </tr>
      </tbody>
    </table>



<!--    <p>Click on the Tauri, Vite, and Vue logos to learn more.</p>-->

<!--    <form class="row" @submit.prevent="greet">-->
<!--      <input id="greet-input" v-model="name" placeholder="Enter a name..." />-->
<!--      <button type="submit">Greet</button>-->
<!--    </form>-->
<!--    <p>{{ greetMsg }}</p>-->
  </main>
</template>

<!--<style scoped>-->

<!--</style>-->

<style>

  .container {
    width: 90%;
    height: 100vh;
    border: 2px red solid;
    margin-left: auto;
    margin-right: auto;
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .word-list {
    border: 2px blue solid;
    width: 95%;
    height: 25%;
    padding-bottom: 1.5rem;
  }

  .word-list ul {
    font-family: "Segoe UI", sans-serif;
    list-style: none;
    display: flex;
    flex-direction: column;
    flex-wrap: wrap;
    width: 100%;
    height: 100%;

  }

  .word-list ul li {
    font-weight: bold;
    border-left: 3px solid pink;
    padding-left: 1rem;
    flex: 1;
  }

  #grid-word {
    margin: 1rem 0;
    border-collapse: collapse;
    border: 3px solid pink;
    width: 95%;
  }

  td, tr{
    border: 1px solid black;
    text-align: center;
  }


</style>