class A {
  static data = 55
}

let obj = new A()

A.data = 67

console.log(obj.data)