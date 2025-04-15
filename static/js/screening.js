function evaluate() {
    const q1 = document.querySelector('input[name="q1"]:checked');
    const q2 = document.querySelector('input[name="q2"]:checked');
    const q3 = document.querySelector('input[name="q3"]:checked');
    const resultDiv = document.getElementById('result');

    if (!q1 || !q2 || !q3) {
        resultDiv.innerHTML = '<div class="alert alert-warning">请回答所有问题！</div>';
        return;
    }

    let score = 0;
    if (q1.value === 'no') score++;
    if (q2.value === 'no') score++;
    if (q3.value === 'yes') score++;

    if (score >= 2) {
        resultDiv.innerHTML = '<div class="alert alert-danger">您的孩子可能存在语言发育迟缓风险，建议咨询专业人士。</div>';
    } else {
        resultDiv.innerHTML = '<div class="alert alert-success">目前未发现明显问题，但请继续观察。</div>';
    }
}