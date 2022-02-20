import { h } from 'preact';

export function Summary({data}) {
    let items = (data.map(x=> (
         <li>Name: {x.a}</li>
    )));
    return (
        <ul>
            {items}
        </ul>
    );
}