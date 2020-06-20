function get_register_device() {
    $.ajax({
        type: "GET",
        dataType: "json",
        url: "/device/get_register_device",
        contentType: "",
        data: "",
        success: function (json) {
            let table = document.getElementById("device");
            let tr = document.createElement("tr");
            let th1 = document.createElement("th");
            let th2 = document.createElement("th");
            th1.innerText = "token";
            th2.innerText = "操作";
            tr.appendChild(th1);
            tr.appendChild(th2);
            table.appendChild(tr);

            for (let value of json) {
                let tr = document.createElement("tr");
                let token = document.createElement("td");
                let op = document.createElement("td");

                token.innerText = value.token;

                if (value.online) {
                    let command_select = document.createElement("select");
                    let command_option = document.createElement("option");
                    let button = document.createElement("input");

                    command_option.text = "GreenLEDLight";

                    button.type = "button";
                    button.id = value.token;
                    button.onclick = execute;
                    button.value = "执行";

                    command_select.appendChild(command_option);
                    op.appendChild(command_select);
                    op.appendChild(button);
                } else {
                    op.innerText = "设备离线";
                }

                tr.appendChild(token);
                tr.appendChild(op);
                table.appendChild(tr);
            }
        },
        error: function () {
            window.location.href = "/user/login";
        }
    });
}

function execute() {
    let td = this.parentElement;
    let command_select = td.children[0];
    let command_option_index = command_select.selectedIndex;
    let command_option = command_select.children[command_option_index].value;

    $.ajax({
        type: "POST",
        dataType: "text",
        url: "/device/command/" + this.id,
        contentType: "application/json",
        data: JSON.stringify({
            "board": command_option.toString(),
            "esp": "None"
        }),
    });
}

get_register_device()