const sleep = (delay) => new Promise((resolve) => setTimeout(resolve, delay));
const mainText = document.querySelector("#main-text");
const timeText = document.querySelector("#update-text");

function cleanStr(str) {
	return str.replace("\t", "").replace("\n", "").replace("\r", "");
}

async function getPageHtml() {
	let resp = await fetch("/opgg");
	let html = await resp.text();
	var htmlObject = document.createElement('div');
	htmlObject.innerHTML = html;
	return htmlObject;
}

function getMinutesSinceUpdate(htmlObject) {
	let lastUpdate = htmlObject.getElementsByClassName("LastUpdate");
	let timestamp = Number(lastUpdate[0].firstElementChild.dataset.datetime) * 1000;
	var updatedTimestamp = new Date(timestamp);
	var now = new Date();
	let delta = Math.floor((now - updatedTimestamp) / 60000);
	return delta;
}

async function startRenew() {
	let formData = new FormData();
	formData.append('summonerId', '49323983');

	let response = await fetch('/renew', {
	  method: 'POST',
	  body: formData
	});

	let json = await response.json();
	return json;
}

async function renewStatus() {
	let formData = new FormData();
	formData.append('summonerId', '49323983');

	let response = await fetch('/renewStatus', {
	  method: 'POST',
	  body: formData
	});

	let json = await response.json();
	return json;
}

async function renew() {
	// start by kicking off the renew
	let start = await startRenew();
	if (!start.finish) {
		// We'll need to loop until it's done!
		let delayVal = start.delay;
		while (true) {
			await sleep(delayVal);
			let status = await renewStatus();
			if (status.finish) {
				break;
			}
			delayVal = status.delay;
		}
	}
}

function gameOutcome(didWin) {
	if (didWin) {
		return "won";
	}
	return "lost";
}

async function run() {
	let pageHtml = await getPageHtml();
	if (getMinutesSinceUpdate(pageHtml) > 5) {
		await renew();
		pageHtml = await getPageHtml();
	}

	timeText.textContent = 'Last updated ' + getMinutesSinceUpdate(pageHtml) + 'min ago';

	let games = pageHtml.getElementsByClassName("GameItemWrap");
	let lastGame = games[games.length - 1];
	let didWin = lastGame.getElementsByClassName("GameResult")[0].innerText.search("Defeat") == -1;
	let champName = cleanStr(lastGame.getElementsByClassName("ChampionName")[0].innerText);

	mainText.textContent = 'Parker ' + gameOutcome(didWin) + ' his last game where he played ' + champName;
}

run();
