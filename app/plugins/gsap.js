import { Flip } from 'gsap/all';

export default defineNuxtPlugin({
  parallel: true,
  setup() {
    useGSAP().registerPlugin(Flip);

    return {
      provide: { flip: Flip },
    };
  },
});
