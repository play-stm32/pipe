function check() {
    $.ajax({
        type: "GET",
        dataType: "text",
        url: "/check",
        success: function (request) {
            if (request == "OK") {
                window.location.href = "/device";
            }
        }
    });
}

function login() {
    let username = document.getElementById("username").value;
    let password = document.getElementById("password").value;
    $.ajax({
        type: "POST",
        dataType: "text",
        url: "/user/login",
        contentType: "application/json",
        data: JSON.stringify({
            "username": username,
            "password": password
        }),
        success: function (request) {
            window.location.href = request;
        }
    });
}

check()