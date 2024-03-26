function inputValueChanged() {
    const input = document.querySelector('input')
    input.value = input.value.replace(/\D/g, '')
}
document.addEventListener('DOMContentLoaded', () => {
    const form = document.querySelector('form')
    form.addEventListener('submit', event => {
        event.preventDefault()
    })
    form.querySelector('button').addEventListener('click', () => invoke())
    const input = document.querySelector('input')
    input.addEventListener('input', () => inputValueChanged())
    input.addEventListener('afterpaste', () => inputValueChanged())
    const { invoke } = window.__TAURI__.tauri
    invoke('get_ipaddr').then((addrs) => {
        const select = document.querySelector('select')
        const strList = addrs.split(',')
        strList.forEach((addr, _) => {
            const option = document.createElement('option')
            option.innerText = addr
            select.appendChild(option)
        })
    })
})