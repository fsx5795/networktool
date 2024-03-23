document.addEventListener('DOMContentLoaded', () => {
    const { invoke } = window.__TAURI__.tauri
    invoke('get_ipaddr').then((addrs) => {
        const select = document.querySelector('select')
        /*
        const strList = addrs.split(',')
        strList.forEach((addr, _) => {
            const option = document.createElement('option')
            option.innerText = addr
            select.appendChild(option)
        })
        */
        const option = document.createElement('option')
        console.log("------------")
        console.log(addrs)
        option.innerText = addrs
        select.appendChild(option)
    })
})