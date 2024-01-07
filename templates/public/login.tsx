function login() {
    const usernameInput = document.getElementById('username') as HTMLInputElement;
    const passwordInput = document.getElementById('password') as HTMLInputElement;
    const username = usernameInput.value
    const password = passwordInput.value

    const xhr = new XMLHttpRequest();

    xhr.onreadystatechange = function() {
        if (xhr.readyState === XMLHttpRequest.DONE) {
            if (xhr.status === 200) {
                // 登录成功，可以进行后续操作
                console.log('Login successful!');
            } else {
                // 登录失败，处理错误信息
                console.error('Login failed:', xhr.responseText);
            }
        }
    };

    xhr.open('POST', '/login', true);
    xhr.setRequestHeader('Content-Type', 'application/json');

    const data = JSON.stringify({ username, password });
    xhr.send(data);
}