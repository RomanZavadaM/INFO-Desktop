import assert from "node:assert/strict";
import {InfoServerClient} from "../src/protocol";

assert.equal(typeof InfoServerClient.prototype.queueAll,"function");
console.log("queueAll method present");
