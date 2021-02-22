<template>
<div>
  <p>{{ greeting }} World!</p>
  <button v-on:click="printProp">Print Message</button>
</div>
</template>

<script>
module.exports = {
  props: {
    greeting: String,
  },
  data() {
    return {
      greeting: String,
    };
  },

  methods: {
    printProp() {
      console.log(this.greeting);
    },
  },
};
</script>

<style scoped>
p {
  font-size: 2em;
  text-align: center;
}
</style>
