/** @jsx h */
import { h } from 'preact';

import '../static/bootstrap.min.css';
import '../static/shtats.css';
import addDays from '../utils/date';
import {generate_added_deleted_data} from "./utils";
import LinesAddedDeletedByLanguage from "../components/lines_added_deleted_by_language";

export default {
    title: 'Shtats/Lines Added Deleted By Language',
    component: LinesAddedDeletedByLanguage,
};

const Template = (args) => <LinesAddedDeletedByLanguage {...args} />;


function getRandomIntInclusive(min, max) {
    const crypto = window.crypto || window.msCrypto;
    const randomBuffer = new Uint32Array(1);

    crypto.getRandomValues(randomBuffer);

    let randomNumber = randomBuffer[0] / (0xffffffff + 1);

    min = Math.ceil(min);
    max = Math.floor(max);
    return Math.floor(randomNumber * (max - min + 1)) + min;
}

function make_extension() {
    var result           = '';
    var characters       = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    var charactersLength = characters.length;
    for ( var i = 0; i < 2; i++ ) {
        result += characters.charAt(Math.floor(Math.random() *
            charactersLength));
    }
    return result;
}


function generate_data(language_count){
    let returnData = [];
    for(let i  = 0; i < language_count;i++){
        let extension = make_extension();
        if (returnData.indexOf(extension) === -1) {
            returnData.push([extension, getRandomIntInclusive(1,50000), getRandomIntInclusive(1,500)]);
        }
    }
    return returnData;
}

export const WithNoData = Template.bind({});
WithNoData.args = {
    data: {
        stats: []
    },
};

export const WithSomeData = Template.bind({});
WithSomeData.args = {
    data: generate_data(25)
};

export const WithLotsOfData = Template.bind({});
WithLotsOfData.args = {
    data: generate_data(100)
};
