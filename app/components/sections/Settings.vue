<script setup>
const appVersion = ref('')

const checkForUpd = async () => {
  await new Promise(resolve => setTimeout(
    resolve,
    1000,
  ))
  appVersion.value = 'Latest version'
}

const themes = computed(() => [
  {
    id: 'light',
    title: 'Light',
  },
  {
    id: 'dark',
    title: 'Dark',
  },
])
const currentTheme = ref(themes.value[1])

const languages = computed(() => [
  {
    id: 'ru',
    title: 'Ru',
  },
  {
    id: 'en',
    title: 'En',
  },
])
const currentLang = ref(languages.value[1])
</script>

<template>
  <div class="sections-settings">
    <div class="sections-settings__title">
      <p class="h2-r-r">
        Account
      </p>
    </div>

    <div class="sections-settings__content">
      <div class="sections-settings__setting">
        <p class="h4-r-r">
          Credentials
        </p>

        <UiStatus :status="'installed'" />
      </div>

      <div class="sections-settings__setting">
        <p class="h4-r-r">
          Project id
        </p>

        <UiStatus :status="'installed'" />
      </div>
    </div>

    <div class="sections-settings__title">
      <p class="h2-r-r">
        App
      </p>
    </div>

    <div class="sections-settings__content">
      <div class="sections-settings__setting">
        <p class="h4-r-r">
          Theme
        </p>

        <!-- <select class="i2-r-r">
          <option value="light" disabled>Ligth</option>

          <option value="dark" selected>Dark</option>
        </select> -->

        <UiDropdown
          v-model="currentTheme"
          :items="themes"
        />
      </div>

      <div class="sections-settings__setting">
        <p class="h4-r-r">
          Language
        </p>

        <UiDropdown
          v-model="currentLang"
          :items="languages"
        />
      </div>
    </div>

    <div class="sections-settings__title">
      <p class="h2-r-r">
        Google
      </p>
    </div>

    <div class="sections-settings__content">
      <div class="sections-settings__setting">
        <p class="h4-r-r">
          Gcloud CLI
        </p>

        <UiStatus :status="'installed'" />
      </div>
    </div>

    <div class="sections-settings__title">
      <p class="h2-r-r">
        Updates
      </p>
    </div>

    <div class="sections-settings__content">
      <div class="sections-settings__setting">
        <p class="h4-r-r">
          Version
        </p>

        <p class="h4-r-r">
          0.1.0
        </p>
      </div>
    </div>

    <div class="sections-settings__content">
      <div class="sections-settings__setting">
        <button
          class="sections-settings__button"
          @click="checkForUpd"
        >
          <p class="h5-r-r">
            Check for updates
          </p>
        </button>

        <AFade>
          <p
            v-if="appVersion"
            class="h5-r-r"
          >
            {{ appVersion }}
          </p>
        </AFade>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.sections-settings {
  display: flex;
  flex-direction: column;
  gap: em(20);
  max-width: em(740);
  padding-inline: em(20);
  margin-inline: auto;

  @include media-breakpoint-down(sm) {
    max-width: none;
    padding: 0;
  }

  &__title {
    padding-block: em(20);
    border-bottom: 1px solid $border-color-primary;
  }

  &__content {
    display: flex;
    flex-direction: column;
    gap: em(10);
  }

  &__setting {
    display: flex;
    flex-wrap: wrap;
    gap: em(20);
    align-items: center;
    min-height: em(30);

    & > p {
      flex: 1;

      &:last-child {
        text-align: right;
      }
    }
  }

  &__button {
    padding: em(10) em(15);
    background-color: $actions-color-primary;
    border-radius: em(8);
    transition: background-color $time-normal $ease-out;

    @include hover-active-focus {
      background-color: $actions-color-secondary;
    }

    @include active {
      background-color: $actions-color-tertiary;
    }
  }
}
</style>
