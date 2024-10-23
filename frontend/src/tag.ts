import { SearchCallback } from './types.js'

export function setupTag(tagElement: HTMLAnchorElement, callback: SearchCallback) {

    tagElement.addEventListener('click', () => {
        callback(tagElement.innerHTML)
    })

}