const errorMsg = document.querySelector(".error");
const data = document.querySelector("#data");
const input = document.querySelector("#input");
const btn = document.querySelector("#btn");

const initWs = () => {
    if (window.ws) ws.close();
    window.ws = new WebSocket("ws://127.0.0.1:3012");

    ws.onopen = (event) => {
        console.log("CONNECTED");

        ws.onmessage = function (event) {
            console.log(event.data);
            data.innerHTML += `<li>${event.data}</li>`;
            const option = {
                body: event.data
            }
            new Notification("New Message!", option);
        }
    };
}
initWs();

const sendMsg = () => {
    errorMsg.textContent = "";
    const t = input.value;
    if (t && ws.readyState !== ws.CLOSED) {
        try {
            window.ws.send(t);
            input.value = "";
        } catch (err) {
            errorMsg.textContent = `Error on websocket connection: ${error}`;
        }
    } else {
        errorMsg.textContent = `Websocket is closed`;
    }
}
btn.addEventListener("click", () => {
    sendMsg();
})
input.addEventListener("keyup", (event) => {
    event.preventDefault();
    if (event.keyCode === 13) {
        sendMsg();
    }
});

const notifyMe = () => {
    if (!("Notification" in window)) {
        console.error("This browser does not support desktop notification");
    } else if (Notification.permission === "granted") {} else if (Notification.permission !== "denied") {
        Notification.requestPermission().then(function (permission) {
            if (permission === "granted") {
                var notification = new Notification("Thank you!");
            }
        });
    }
}

notifyMe()