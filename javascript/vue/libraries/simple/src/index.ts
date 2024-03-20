import type { App } from 'vue';
import components from './components';

const plugin = {
  install(app: App) {
    for (const component of Object.values(components)) {
      app.component(component.name, component);
    }
  },
};

export default plugin;
