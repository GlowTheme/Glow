import os
import sys
import math
from datetime import datetime, timedelta
from collections import defaultdict, namedtuple
from typing import List, Dict, Union, Optional

class MyClass:
    """A simple example class"""

    class_variable = "Class Variable"

    def __init__(self, name: str, value: int = 42):
        self.name = name
        self.value = value

    def greet(self) -> str:
        return f"Hello, {self.name}!"

    @staticmethod
    def static_method() -> str:
        return "This is a static method"

    @classmethod
    def class_method(cls):
        return f"Class method called from {cls.__name__}"

def example_function(arg1: int, arg2: str) -> Dict[str, Union[int, str]]:
    """Example function demonstrating basic operations"""
    result = {'arg1': arg1, 'arg2': arg2}
    if arg1 > 10:
        print("arg1 is greater than 10")
    elif arg1 == 10:
        print("arg1 is equal to 10")
    else:
        print("arg1 is less than 10")

    for i in range(arg1):
        print(f"Loop {i}: {arg2}")

    try:
        division_result = arg1 / (arg1 - 10)
    except ZeroDivisionError as e:
        print(f"Caught an exception: {str(e)}")
        division_result = None

    return result

def decorator_function(func):
    """Example of a decorator"""
    def wrapper(*args, **kwargs):
        print(f"Calling function {func.__name__}")
        return func(*args, **kwargs)
    return wrapper

@decorator_function
def decorated_function(x: int):
    return x * 2

if __name__ == "__main__":
    my_object = MyClass("VSCode")
    print(my_object.greet())

    example_dict = example_function(15, "Test String")
    print(f"Example dict: {example_dict}")

    double_value = decorated_function(21)
    print(f"Double value: {double_value}")

    with open("example.txt", "w") as file:
        file.write("This is a test file.\n")

    numbers = [1, 2, 3, 4, 5]
    squares = [n ** 2 for n in numbers]
    print(f"Squares: {squares}")

    # Lambda function example
    add = lambda a, b: a + b
    print(f"Lambda add: {add(10, 20)}")

    # Example of a generator
    def simple_generator():
        yield 1
        yield 2
        yield 3

    for value in simple_generator():
        print(f"Generator value: {value}")

    # Working with datetime
    now = datetime.now()
    future_date = now + timedelta(days=5)
    print(f"Future date: {future_date}")

    # Example with defaultdict
    def_dict = defaultdict(lambda: "default value")
    def_dict['key1'] = "value1"
    print(def_dict['key1'])
    print(def_dict['non_existent_key'])

    # Example with namedtuple
    Point = namedtuple('Point', ['x', 'y'])
    p = Point(10, 20)
    print(f"Point: {p.x}, {p.y}")

    # List comprehension with conditionals
    evens = [x for x in range(20) if x % 2 == 0]
    print(f"Evens: {evens}")

    # Example with typing
    def process_items(items: List[int]) -> Optional[int]:
        if not items:
            return None
        return sum(items)
    
    print(process_items([1, 2, 3, 4, 5]))
    print(process_items([]))

    # F-strings with expressions
    name = "VSCode"
    version = 1.57
    print(f"{name} version: {version * 2}")

    # Example of using os and sys
    current_dir = os.getcwd()
    python_version = sys.version
    print(f"Current directory: {current_dir}")
    print(f"Python version: {python_version}")
