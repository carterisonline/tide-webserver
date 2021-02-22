<template>
  <div>
    <Navbar></Navbar>
    <Post greeting="be"></Post>
  </div>
</template>

<script>
import Navbar from './components/Navbar.vue';
import Post from './components/Post.vue';

export default {
  components: {
    Navbar,
    Post,
  },
};
</script>
