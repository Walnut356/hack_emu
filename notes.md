# Notes

book treats *bit* arrays as little endian (i.e. least significant *bit* first). For selector bits, this means 10 = s[0, 1]

---

* `function()`

is interpreted as the method of the current class


* `identifier.function()`

If the identifier **is** found in the symbol table, the call is assumed to be a **method**.

If the identifier is **not** found, the call is assumed to be a class **function**.
