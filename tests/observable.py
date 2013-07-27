# -*- coding: utf-8 -*-
from unittest import TestCase

from sublimate import observable

class ObservableTestCase(TestCase):

    def test_creation(self):
        class Foo(observable.Object):
            a = observable.Attribute()
        assert len(Foo._observables) == 1
        assert len(Foo()._observables) == 1
        class Bar(Foo):
            b = observable.Attribute()
            c = 42
        assert len(Bar._observables) == 2
        assert len(Bar()._observables) == 2


    def test_attr(self):
        class Foo(observable.Object):
            a = observable.Attribute()
        foo = Foo()
        assert foo.a is None
        foo.a = 42
        assert foo.a == 42
        assert Foo().a is None
        class Foo(observable.Object):
            a = observable.Attribute(23)
        foo = Foo()
        assert foo.a == 23

    def test_computed(self):
        class Foo(observable.Object):
            calls = 0
            a = observable.Attribute(5)
            b = observable.Attribute(7)
            call_count = 0
            CONST = 4

            @observable.computed
            def c(self):                
                self.calls += 1
                return self.a + self.b

            @observable.computed
            def e(self):
                return self.c * 2

            @observable.computed
            def d(self):
                return self.CONST + 1

        foo = Foo()
        assert foo.d == 5
        assert foo.a == 5, foo.b == 7
        assert foo.c == 12
        assert foo.e == 24
        foo.a = 1
        assert foo.c == 8
        assert foo.e == 16
        foo.b = 2
        assert foo.c == 3        
        assert foo.e == 6
        assert foo.calls == 3

    def test_recall(self):
        class Foo(observable.Object):
            a = observable.Attribute(5)
            calls = 0

            @observable.recall
            def b(self):
                self.calls += 1

        foo = Foo()
        assert foo.calls == 0
        foo.b()
        assert foo.calls == 1
        foo.a = 3
        assert foo.calls == 2
        foo.a = 3
        assert foo.calls == 2
