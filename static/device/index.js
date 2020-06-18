function get_register_device() {
    $.ajax({
        type: "GET",
        dataType: "text",
        url: "/device/get_register_device",
        contentType: "",
        data: "",
        success: function (request) {
            let table = document.getElementById("device");
            let tr = document.createElement('tr');
            let token = document.createElement('td');
            token.innerText = request;

            let operation = document.createElement('td');
            let type_select = document.createElement("select");
            let type_option = document.createElement("option");
            type_option.text = "Board";

            let command_select = document.createElement("select");
            let command_option = document.createElement("option");
            command_option.text = "GreenLedLight";

            let button = document.createElement("input");
            button.type = "button"
            button.onclick = execute;
            button.value = "执行";

            type_select.appendChild(type_option);
            command_select.appendChild(command_option);

            operation.appendChild(type_select);
            operation.appendChild(command_select);
            operation.appendChild(button);

            tr.appendChild(token);
            tr.appendChild(operation);
            table.appendChild(tr);
        }
    });
}

function execute() {
    $.ajax({
        type: "POST",
        dataType: "text",
        url: "/device/command/xxx",
        contentType: "application/json",
        data: JSON.stringify({
            "board": "GreenLEDLight",
            "esp": "None"
        }),
    });
}

get_register_device()