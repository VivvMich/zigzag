<script setup lang="ts">
import { ref } from "vue";
import {invoke} from "@tauri-apps/api/core";

const gretMsg = ref("");
// const name = ref("");

const rows = ref([])

function generateTable() {
  const width = 31
  const height = 29

  const holeW = 9
  const holeH = 11

  // marges calculées
  const marginX = Math.floor((width - holeW) / 2)   // 11
  const marginY = Math.floor((height - holeH) / 2)  // 9

  const table = []

  for (let y = 0; y < height; y++) {
    const row = []

    for (let x = 0; x < width; x++) {
      const insideHole =
          y >= marginY &&
          y < marginY + holeH &&
          x >= marginX &&
          x < marginX + holeW

      row.push(insideHole ? "" : "#")
    }

    table.push(row)
  }

  rows.value = table
}

async function getWords(): Promise<string[]> {
   let words: string[]  = await invoke("get_words");
   return words;
}

getWords();
generateTable();
</script>

<template>
  <main class="container">
    <h1>Générateur de mot en vrac</h1>
    <div class="word-list">
      <ul>
        <li>Lorem.</li>
        <li>Aliquid.</li>
        <li>Velit.</li>
        <li>Expedita.</li>
        <li>Totam!</li>
        <li>Cum.</li>
        <li>Qui.</li>
        <li>Molestiae.</li>
        <li>Deserunt!</li>
        <li>Laudantium?</li>
        <li>Porro.</li>
        <li>Consectetur!</li>
        <li>Voluptas?</li>
        <li>Cumque!</li>
        <li>Consequatur.</li>
        <li>Molestias!</li>
        <li>Culpa!</li>
        <li>Nemo.</li>
        <li>Tempora?</li>
        <li>Voluptatem!</li>
        <li>Nesciunt?</li>
        <li>Dolorum!</li>
        <li>Exercitationem!</li>
        <li>Doloremque?</li>
        <li>Cum!</li>
        <li>Culpa.</li>
        <li>Et.</li>
        <li>Nesciunt!</li>
        <li>Optio!</li>
        <li>Labore.</li>
        <li>Ipsum!</li>
        <li>Delectus.</li>
        <li>Rerum.</li>
        <li>Iste?</li>
        <li>Incidunt?</li>
        <li>Molestiae.</li>
        <li>Libero.</li>
        <li>Eaque?</li>
        <li>Quidem.</li>
        <li>Ducimus.</li>
        <li>Inventore.</li>
        <li>Error.</li>
        <li>Pariatur.</li>
        <li>Facere.</li>
        <li>Ab!</li>
        <li>Harum.</li>
        <li>Maxime.</li>
        <li>Assumenda?</li>
        <li>Animi?</li>
        <li>At.</li>
        <li>Qui?</li>
        <li>Doloribus.</li>
        <li>Vitae?</li>
        <li>Corporis!</li>
        <li>Laborum.</li>
        <li>Dolorem.</li>
        <li>Architecto?</li>
        <li>Nemo.</li>
        <li>Sint!</li>
        <li>Porro!</li>
        <li>Saepe!</li>
        <li>Quidem.</li>
        <li>Dolores!</li>
        <li>Commodi.</li>
        <li>Nihil.</li>
        <li>Illum?</li>
        <li>Quis!</li>
        <li>Sapiente.</li>
        <li>Dolorem?</li>
        <li>Porro!</li>
        <li>Cumque.</li>
        <li>Impedit.</li>
        <li>Odit!</li>
        <li>Perspiciatis?</li>
        <li>Adipisci.</li>
        <li>Quam?</li>
        <li>Ut!</li>
        <li>Consequatur.</li>
        <li>Quaerat.</li>
        <li>Accusamus.</li>
        <li>Optio.</li>
        <li>Laborum?</li>
        <li>Modi.</li>
        <li>Maxime.</li>
        <li>Placeat.</li>
        <li>Aspernatur?</li>
        <li>Labore!</li>
        <li>Veniam!</li>
        <li>Molestiae?</li>
        <li>Illo.</li>
        <li>Reprehenderit.</li>
        <li>Culpa?</li>
        <li>Nobis.</li>
        <li>Mollitia.</li>
        <li>Recusandae.</li>
        <li>Maxime?</li>
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