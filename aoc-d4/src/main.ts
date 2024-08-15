
function ex4Part2(data: string) {

    let ind: number[] = [];

    for (let pas = 0; pas < 214; pas++) {
        ind[pas] = 1;
    }
	let i = 0;
    data.toString().split("\n").forEach((e: any) => {

        let cardArray: string[] = e.split(": ");
        let cardGame: string[] = cardArray[1].split(" | ");
        let cardNumber: number = Number(cardArray[0].replace(/\D/g, ''));

        let intersecPrepa = cardGame.map((e) => e.replace(/  /g, ' ').replace("\r", '').trim().split(' '));
        const matches = intersecPrepa[0].filter(value => intersecPrepa[1].includes(value)).length;

        for (let j = i+1; j < i+matches+1; j++) {
            ind[j] += ind[i];
        }


		i++;
    });

	let sum = 0
    for (let j = 0; j < 214; j++) {
        sum += ind[j];
    }

	console.log(j);
}