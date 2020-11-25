async function getPageHtml() {
	let resp = await fetch("https://didparkerwinhislastgame.netlify.app/opgg");
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
	let url = new URL('https://didparkerwinhislastgame.netlify.app/renew');
	url.searchParams.set('summonerId', '49323983');

	let response = await fetch(url, {
	  method: 'POST'
	});
	return response;
}


async function renewStatus() {
	let url = new URL('https://didparkerwinhislastgame.netlify.app/renewStatus');
	url.searchParams.set('summonerId', '49323983');

	let response = await fetch(url, {
	  method: 'POST'
	});
	return response;
}