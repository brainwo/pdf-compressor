import './app.css'
import App from './App.svelte'
import { CompressPdf }  from 'pdf-compressor-lib';

const app = new App({
  target: document.getElementById('app')
})

export default app
