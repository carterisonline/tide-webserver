<template>
  <div>
    <Navbar></Navbar>
    <div style="top:0px;
              padding-top:6%;
              margin:auto; position:relative;
              width:950px;
              height:100%;">
    <b-form @submit="onSubmit" @reset="onReset" v-if="show">
      <b-form-group
        id="input-group-1"
        label="Email address:"
        label-for="input-1"
        description="We'll absolutely share your email with everybody else."
      >
        <b-form-input
          id="input-1"
          v-model="form.email"
          type="email"
          placeholder="Enter email"
          required
        ></b-form-input>
      </b-form-group>

      <b-form-group id="input-group-2" label="Your Name:" label-for="input-2">
        <b-form-input
          id="input-2"
          v-model="form.name"
          placeholder="Enter name"
          required
        ></b-form-input>
      </b-form-group>

      <b-form-group id="input-group-3" label="Favorite Amine:" label-for="input-3">
        <b-form-select
          id="input-3"
          v-model="form.amine"
          :options="amine"
          required
        ></b-form-select>
      </b-form-group>

      <b-form-group id="input-group-4" v-slot="{ ariaDescribedby }">
        <b-form-checkbox-group
          v-model="form.checked"
          id="checkboxes-4"
          :aria-describedby="ariaDescribedby"
        >
          <b-form-checkbox value="me">Check me out</b-form-checkbox>
          <b-form-checkbox value="that">Check that out</b-form-checkbox>
        </b-form-checkbox-group>
      </b-form-group>

      <b-button type="submit" variant="primary">Submit</b-button>
      <b-button type="reset" variant="danger">Reset</b-button>
    </b-form>
    <b-card class="mt-3" header="Form Data Result">
      <pre class="m-0">{{ form }}</pre>
    </b-card>
    </div>
  </div>
</template>

<script>
import Navbar from './components/Navbar.vue';

export default {
  components: {
    Navbar,
  },
  data() {
    return {
      form: {
        email: '',
        name: '',
        amine: null,
        checked: [],
      },
      amine: [{ text: 'Select One', value: null }, 'JoJo\'s big quest', 'Nutty noodle man', 'KanyeCon', '932Piece'],
      show: true,
    };
  },
  methods: {
    onSubmit(event) {
      event.preventDefault();
      // Should probably use: https://bootstrap-vue.org/docs/components/alert
      alert(JSON.stringify(this.form));
    },
    onReset(event) {
      event.preventDefault();
      // Reset our form values
      this.form.email = '';
      this.form.name = '';
      this.form.amine = null;
      this.form.checked = [];
      // Trick to reset/clear native browser form validation state
      this.show = false;
      this.$nextTick(() => {
        this.show = true;
      });
    },
  },
};
</script>
