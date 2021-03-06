<template>
  <div class="file-list">
    <header class="item">
      Touist <span class="version">{{ config.version }}</span>
    </header>
    <span class="item new-file" @click="createNewFile"><i class="fa fa-file-o fa-fw"></i> Create new file</span>
    <router-link
      class="item"
      :class="{ selected: openFile && file.name === openFile.name }"
      v-for="file in files"
      :to="{ name: 'editfile', params: { name: file.name }}"
      :key="file.name"
    >
      <i class="fa fa-file-text fa-fw"></i> {{ file.name }}
      <i class="fa fa-angle-right open-action"></i>
    </router-link>
    <div style="flex-grow: 1"></div>
    <a class="item report-bug" target="_blank" rel="noopener noreferrer" href="https://github.com/graphman65/touist-editor/issues">
      <i class="fa fa-bug fa-fw"></i> Report bug
    </a>
  </div>
</template>

<script>
import { mapState, mapGetters, mapMutations } from 'vuex';

import config from '@/config';
import nextId from '@/utils/nextId';

export default {
  name: 'FileList',
  data: () => ({
    config,
  }),
  methods: {
    ...mapMutations(['newFile']),
    createNewFile() {
      const name = `File ${nextId()}`;
      this.newFile(name);
      this.$router.replace(`/editor/${name}`);
    },
  },
  computed: {
    ...mapState(['files']),
    ...mapGetters(['openFile']),
  },
};
</script>

<style lang="scss">
@import '../assets/variables.scss';

.file-list {
  height: 100%;
  width: 100%;
  background-color: $dark_color;
  display: flex;
  flex-direction: column;
  color: #f2f1ef;

  .item {
    font-size: 14px;
    user-select: none;
    padding: 10px;
    text-decoration: none;
    color: $light_color;
    cursor: pointer;

    &.selected {
      background-color: rgba(0, 0, 0, 0.2);
      cursor: default;
    }

    .open-action {
      display: none;
      float: right;
    }

    &:not(.selected):hover {
      .open-action {
        display: inline;
      }
    }

    &.new-file {
      // margin-top: 20px;
      background-color: $secondary_color;
    }

    &.report-bug {
      background-color: $ternary_color;
    }
  }

  header {
    height: 40px;
    padding: 0px;
    display: flex;
    align-items: center;
    justify-content: center;
    text-align: center;
    text-transform: uppercase;
    font-weight: bold;
    background-color: rgba(0, 0, 0, 0.5);
    box-shadow: 0 0 5px rgba(0, 0, 0, 0.5);
  }

  .version {
    font-weight: normal;
    text-transform: none;
  }
}
</style>
