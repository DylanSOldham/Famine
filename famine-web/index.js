import init, {web_startup, web_update} from "./pkg/famine_web.js"

let application
init().then(() => {
    web_startup().then(
        app => setInterval(web_update, 33, app)
    )
})