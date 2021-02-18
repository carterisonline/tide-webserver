// index.js
import { BootstrapVue, IconsPlugin } from 'bootstrap-vue';

import Vue from 'vue';
import App from './App.vue';

import 'bootstrap/dist/css/bootstrap.css';
import 'bootstrap-vue/dist/bootstrap-vue.css';

Vue.use(BootstrapVue);
Vue.use(IconsPlugin);

new Vue({ render: (createElement) => createElement(App) }).$mount('#app');
