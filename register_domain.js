const url = 'https://icp0.io/registrations';
const data = {
    name: "www.aublisk.com"
};

fetch(url, {
    method: 'POST',
    headers: {
        'Content-Type': 'application/json'
    },
    body: JSON.stringify(data)
})
.then(response => {
    console.log('Status:', response.status);
    return response.text();
})
.then(text => {
    console.log('Response:', text);
})
.catch(error => {
    console.error('Error:', error);
});