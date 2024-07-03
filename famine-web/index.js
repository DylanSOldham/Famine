import init, {web_startup, web_update} from "./pkg/famine_web.js"

let application
init().then(() => {
    application = web_startup()
    setInterval(web_update, 33, application)
})