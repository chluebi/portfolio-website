import './style.css'
import { setupCounter } from './searchbar.ts'


setupCounter(document.querySelector<HTMLInputElement>('#searchBox')!)
