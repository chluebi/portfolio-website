import { searchCallback } from './types.ts'

export function setupTag(tagElement: HTMLAnchorElement, callback: searchCallback) {

    tagElement.addEventListener('click', () => {
        callback(tagElement.innerHTML)
    })

}