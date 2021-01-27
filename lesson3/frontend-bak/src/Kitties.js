import React, { useEffect, useState } from 'react';
import { Form, Grid } from 'semantic-ui-react';

import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';

import KittyCards from './KittyCards';

export default function Kitties (props) {
  const { api, keyring } = useSubstrate();
  const { accountPair } = props;

  const [kittyIndex, setKittyIndex] = useState(0);
  const [kittyCnt, setKittyCnt] = useState(0);
  const [kittyDNAs, setKittyDNAs] = useState([]);
  const [kittyOwners, setKittyOwners] = useState([]);
  const [kittyPrices, setKittyPrices] = useState([]);
  const [kitties, setKitties] = useState([]);
  const [status, setStatus] = useState('');

  const fetchKittyCnt = async () => {
    /* TODO: 加代码，从 substrate 端读取数据过来 */
    try {
      const count = await api.query.kittiesModule.kittiesCount();
      console.log('result', count.toNumber());
      setKittyCnt(count.toNumber());
    } catch (e) {
      console.error(e);
    }
  };

  const fetchKitties = async () => {
    /* TODO: 加代码，从 substrate 端读取数据过来 */
    const kitty = await api.query.kittiesModule.kitties(kittyIndex);
    console.log('kitty: ', kitty);
    if (kitty.isNone) {
      console.log('none');
      setKitties('<None>');
    } else {
      console.log('get: ', kitty.unwrap().toString());
      setKitties(kitty.unwrap().toString());
    }
  };

  const populateKitties = async () => {
    /* TODO: 加代码，从 substrate 端读取数据过来 */
    // const result = await api.query.kittiesModule.create();
    // console.log('result: ', result);
  };

  useEffect(fetchKittyCnt, [api, keyring]);
  useEffect(fetchKitties, [kittyIndex, api, kittyCnt]);
  useEffect(populateKitties, [kittyDNAs, kittyOwners]);

  return <Grid.Column width={16}>
    <h1>小毛孩</h1>
    <KittyCards kitties={kitties} accountPair={accountPair} setStatus={setStatus}/>
    <Form style={{ margin: '1em 0' }}>
      <Form.Field style={{ textAlign: 'center' }}>
        <TxButton
          accountPair={accountPair} label='创建小毛孩' type='SIGNED-TX' setStatus={setStatus}
          attrs={{
            palletRpc: 'kittiesModule',
            callable: 'create',
            inputParams: [],
            paramFields: []
          }}
        />
      </Form.Field>
    </Form>
    <div style={{ overflowWrap: 'break-word' }}>{status}</div>
  </Grid.Column>;
}
