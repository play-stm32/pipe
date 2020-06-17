function login() {
    var username = document.getElementById("username").value;
    var password = document.getElementById("password").value;
    $.ajax({
        type: "POST",
        dataType: "text",
        url: '/user/login',
        contentType: "application/json",
        data: JSON.stringify({
            "username": username,
            "password": password
        }),
        success: function (result) {
            alert(result);
        }
    });
}