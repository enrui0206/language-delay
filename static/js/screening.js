// screening data and functions
const questionsByAge = {
    '1-2': [
        '您的孩子是否能说出至少 50 个单词？（如“妈妈”、“吃饭”）',
        '您的孩子是否能组成 2-3 词的短句？（如“我要水”）',
        '您的孩子是否能指认常见物体？（如指着球说“球”）',
        '您的孩子是否能跟随简单指令？（如“把玩具拿过来”）',
        '您的孩子是否会用语言表达需求？（如说“饿”表示想吃饭）',
        '您的孩子是否会模仿您的话？（如您说“拜拜”，孩子也说“拜拜”）'
    ],
    '2-3': [
        '您的孩子是否能说出 200 个以上的单词？',
        '您的孩子是否能用短句表达需求？（如“我要喝水”）',
        '您的孩子是否能使用代词？（如“我”、“你”）',
        '您的孩子是否能回答简单问题？（如“这是什么？”）',
        '您的孩子是否能被陌生人听懂一半以上的话？',
        '您的孩子是否会用语言描述自己的行为？（如“宝宝跑”）'
    ],
    '3-4': [
        '您的孩子是否能用 4-5 词的句子表达？（如“我想要吃苹果”）',
        '您的孩子是否能被陌生人听懂 75% 以上的话？',
        '您的孩子是否能使用简单的语法？（如“他在跑”）',
        '您的孩子是否能讲述简单故事？（如“狗跑了”）',
        '您的孩子是否能回答“为什么”问题？（如“为什么要去睡觉？”）',
        '您的孩子是否能参与简单对话？（如问答 2-3 轮）'
    ],
    '4-5': [
        '您的孩子是否能用完整的句子表达？（如“妈妈，我可以去公园吗？”）',
        '您的孩子是否能被陌生人完全听懂？',
        '您的孩子是否能讲清楚过去的事情？（如“昨天我去了动物园”）',
        '您的孩子是否能使用复杂语法？（如“因为我累了，所以想睡觉”）',
        '您的孩子是否能理解抽象概念？（如“明天我们去哪里？”）',
        '您的孩子是否能参与长时间对话？（如 4-5 轮问答）'
    ]
};

const cutoffScores = {
    '1-2': 25,
    '2-3': 30,
    '3-4': 35,
    '4-5': 40
};

// ensure it can be accessed globally
window.evaluate = function() {
    console.log("evaluate() function called!");
    const ageYearsElement = document.getElementById('age_years');
    const ageMonthsElement = document.getElementById('age_months');
    const resultDiv = document.getElementById('result');

    if (!ageYearsElement || !ageMonthsElement) {
        console.error("Age input elements not found");
        if (resultDiv) {
            resultDiv.innerHTML = '<div class="alert alert-danger">页面加载错误，请刷新重试。</div>';
        }
        return;
    }

    if (!resultDiv) {
        console.error("Result element not found");
        return;
    }

    const ageYears = parseInt(ageYearsElement.value);
    const ageMonths = parseInt(ageMonthsElement.value);
    if (isNaN(ageYears) || isNaN(ageMonths)) {
        console.error("Invalid age input");
        resultDiv.innerHTML = '<div class="alert alert-warning">请输入有效的年龄。</div>';
        return;
    }

    const totalMonths = ageYears * 12 + ageMonths;
    console.log("totalMonths:", totalMonths);

    let ageGroup;
    if (totalMonths >= 12 && totalMonths < 24) {
        ageGroup = '1-2';
    } else if (totalMonths >= 24 && totalMonths < 36) {
        ageGroup = '2-3';
    } else if (totalMonths >= 36 && totalMonths < 48) {
        ageGroup = '3-4';
    } else if (totalMonths >= 48 && totalMonths <= 60) {
        ageGroup = '4-5';
    } else {
        console.log("Invalid age range");
        resultDiv.innerHTML = '<div class="alert alert-warning">请输入 1-5 岁之间的年龄。</div>';
        return;
    }
    console.log("ageGroup:", ageGroup);

    const questions = document.querySelectorAll('#screeningForm input[type="radio"]:checked');
    console.log("Number of questions answered:", questions.length);

    if (questions.length !== 6) {
        console.log("Not all questions answered");
        const unanswered = [];
        for (let i = 0; i < 6; i++) {
            const answered = document.querySelector(`input[name="q${i}"]:checked`);
            if (!answered) unanswered.push(i + 1);
        }
        resultDiv.innerHTML = `<div class="alert alert-warning">请回答所有问题！您未回答第 ${unanswered.join(", ")} 题。</div>`;
        return;
    }

    let score = 0;
    questions.forEach(q => {
        if (q.value === 'yes') score += 10;
        else if (q.value === 'sometimes') score += 5;
    });
    console.log("Score:", score);

    const cutoff = cutoffScores[ageGroup];
    if (typeof cutoff === 'undefined') {
        console.error("Cutoff score not defined for age group:", ageGroup);
        resultDiv.innerHTML = '<div class="alert alert-danger">评分错误，请联系管理员。</div>';
        return;
    }
    console.log("Cutoff:", cutoff);

    let resultText = '';
    if (score >= cutoff + 15) {
        resultText = '<div class="alert alert-success">您的孩子语言发育正常，请继续观察和支持其语言发展。</div>';
    } else if (score >= cutoff) {
        resultText = '<div class="alert alert-warning">您的孩子可能存在轻微语言发育迟缓，建议多与其互动，创造语言环境，并持续观察。</div>';
    } else {
        resultText = '<div class="alert alert-danger">您的孩子可能存在语言发育迟缓风险，建议尽早咨询专业语言治疗师或儿科医生。</div>';
    }

    resultText += `
        <h5>建议与资源</h5>
        <ul>
            <li>多与孩子互动，讲故事、唱歌，提供丰富的语言刺激。</li>
            <li>观察孩子的听力是否正常，听力问题可能影响语言发育。</li>
            <li>了解更多：<a href="https://www.cdc.gov/ncbddd/actearly/milestones/index.html" target="_blank">CDC 发育里程碑</a></li>
            <li>专业资源：<a href="https://www.asha.org/public/speech/development/" target="_blank">美国言语-语言-听力协会 (ASHA)</a></li>
        </ul>
    `;

    console.log("Displaying result");
    resultDiv.innerHTML = resultText;
};

function loadQuestions() {
    console.log("loadQuestions() called");
    const ageYears = parseInt(document.getElementById('age_years').value);
    const ageMonths = parseInt(document.getElementById('age_months').value);
    const totalMonths = ageYears * 12 + ageMonths;
    console.log("totalMonths:", totalMonths);

    let ageGroup;
    if (totalMonths >= 12 && totalMonths < 24) {
        ageGroup = '1-2';
    } else if (totalMonths >= 24 && totalMonths < 36) {
        ageGroup = '2-3';
    } else if (totalMonths >= 36 && totalMonths < 48) {
        ageGroup = '3-4';
    } else if (totalMonths >= 48 && totalMonths <= 60) {
        ageGroup = '4-5';
    } else {
        console.log("Invalid age range");
        document.getElementById('questions').innerHTML = '<p class="text-danger">请输入 1-5 岁之间的年龄。</p>';
        document.getElementById('screeningForm').style.display = 'none';
        return;
    }

    const questionsDiv = document.getElementById('questions');
    questionsDiv.innerHTML = '';
    questionsByAge[ageGroup].forEach((question, index) => {
        const qDiv = document.createElement('div');
        qDiv.className = 'mb-3';
        qDiv.innerHTML = `
            <label class="form-label">${index + 1}. ${question}</label>
            <div>
                <input type="radio" name="q${index}" value="yes" id="q${index}_yes"> <label for="q${index}_yes">是</label>
                <input type="radio" name="q${index}" value="sometimes" id="q${index}_sometimes"> <label for="q${index}_sometimes">有时</label>
                <input type="radio" name="q${index}" value="no" id="q${index}_no"> <label for="q${index}_no">还没有</label>
            </div>
        `;
        questionsDiv.appendChild(qDiv);
    });
    console.log("Questions loaded");
    document.getElementById('screeningForm').style.display = 'block';
}

// dynamically bind event to submit button
document.addEventListener('DOMContentLoaded', () => {
    console.log("DOM fully loaded, binding submit button event");

    const submitButton = document.getElementById('submitButton');
    if (submitButton) {
        submitButton.addEventListener('click', () => {
            console.log("Submit button clicked, calling evaluate()");
            window.evaluate();
        });
    } else {
        console.error("Submit button not found");
    }

    function updateAgeMonths() {
        const monthsSelect = document.getElementById('age_months');
        monthsSelect.innerHTML = '';
        for (let i = 0; i < 12; i++) {
            const option = document.createElement('option');
            option.value = i;
            option.text = i;
            monthsSelect.appendChild(option);
        }
    }

    updateAgeMonths();
    loadQuestions();
});