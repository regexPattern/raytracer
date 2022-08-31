# Raytracing

## Definiciones

* Rendering: Todo el proceso de generacion de una imagen a partir de un modelo
  bi o tridimensional.

* Scaline: Son algoritmos que permiten el renderizado de imagenes que va de
  linea en linea, en vez de poligono por poligono o pixel por pixel. Primero se
  determinan los extremos de los poligonos que se van a renderizar de arriba
  hacia abajo (por su coordenada `y`) para que luego cada file (o "scan line")
  sea computada utilizando la interseccion (linear interpolation). Asi se va
  avanzando por la imagen.

  - https://youtu.be/t7Ztio8cwqM

* Ray Casting: La idea de este algoritmo es similar un rayo pixel por pixel,
  provenientes desde los ojos, y encontrar el objecto mas cercano que bloquee
  la trayectoria del rayo (primer punto de interseccion). Este algoritmo
  facilita el lidear con cuerpos tridimensionales no planos, respecto a
  algoritmos de renderizado previos como scaline.

  Solamente se refiere a la accion de simular el lanzado de rayos desde el
  punto del observador para que estos intersecten a las figuras geometricas de
  la escena.

* Ray Tracing: Hacer seguimiento (tracing) de estos rayos lanzados con la
  intension de conocer sus intersecciones y rebotes con las figuras geometricas
  para ver como estos afectan al rayo en su proxima interseccion.

  Particularmente, "Recursive Ray Tracing", agrega la posibilidad de que cuando
  un rayo impacta, tambien se computan su reflexion, refraccion y shadows.

* Shading: Percepcion de profundidad de un objeto. Se computa utlizando las
  propiedades del material el efecto de la luz en la escena. (Mas sobre esto
  luego).

## Intersections

* Ray: Es una recta, representado generalmente de forma parametrica con un
  vector director y un punto de paso.

> NOTA: Cada cuerpo tiene una formula diferente para computar la interseccion
> de su superficie con un rayo.

### Line-Sphere Intersection

  Asumimos que toda esfera esta centrada en el origen y que tiene radio 1.

  Existen tres posibles resultados de una interseccion entre una esfera y un
  rayo:

  1. Que el rayo atraviese la esfera, resultando en dos puntos de interseccion.
  2. Que el rayo intersecte la esfera de manera tangente, resultando en un
	 unico punto de interseccion.
  3. Que el rayo no intersecte la esfera del todo.

  Basicamente consideramos diferentes vectores, como el que va del origen del
  rayo hasta la esfera, su proyeccion, la distancia entre estos dos vectores,
  etc, etc, y terminamos llegando a una ecuacion cuadratica, cuyo determinante
  nos permite saber si tenemos o no intersecciones:

  - det < 0: No hay intersecciones.
  - det = 0: Hay solo una interseccion.
  - det > 0: Hay dos intersecciones.

  Las soluciones a la ecuacion cuadratica son los puntos de interseccion.

  - https://www.lighthouse3d.com/tutorials/maths/ray-sphere-intersection/
  - https://www.youtube.com/watch?v=5ZHh8vUcEak
  - https://www.nagwa.com/en/explainers/987161873194/
  - https://www.khanacademy.org/math/algebra/x2f8bb11595b61c86:quadratic-functions-equations/x2f8bb11595b61c86:quadratic-formula-a1/a/discriminant-review
  - https://gregorycernera.medium.com/an-explanation-of-basic-ray-tracing-313373c852ac#:~:text=If%20the%20discriminant%20is%20negative,will%20be%20two%20hit%20points.

### Transforming Rays and Spheres

Basicamente lo que nos tenemos que plantear para transformar las esferas, es
mas bien transformar los rayos que impactan en estas esferas. Esto porque
nuestro algoritmo de interseccion tiene como supuesto que nuestra esfera esta
ubicada en el centro y tiene radio 1 siempre. Para aplicar al rayo el efecto
que se le aplica a la esfera, debemos utilizar la inversa de esta
transformacion de la esfera pero sobre el rayo.

### Phong's Reflection Model

Solamente necesitamos cuatro vectores para simular sombras (todos salientes
desde el punto en cuestion:

- E: Vector que se dirige hacia el ojo/camara.
- N: Vector normal a la superficie en ese punto.
- R: Vector de reflexion.
- L: Vector que se dirige hacia la fuente de luz.

Cuando queremos calcular la normal de una esfera en condiciones ideales
simplemente restamos el punto en cuestion con el centro de la esfera. Pero
luego de que aplicamos transformaciones a esta esfera, tenemos que tomar estas
transformaciones en cuenta. Para eso tenemos que convertir tanto el punto en
cuestion como la normal a coordenadas del "object space".

> NOTA: Parece ser que para que dos objetos independientes interaccionen
> necesitamos hacerlo a traves del "world space".

### Resumen Intersections

1. Se lanza un rayo y se identifica si esta ha intersectado con la esfera o no.
   Para esto tenemos que convertir las coordenadas de ambos a world space.
2. Si tenemos un hit, entonces producimos un color. Para esto vamos a usar el
   Phong's Reflection Model, por lo que vamos a necesitar el vector desde la
   camara hasta el punto de hit, la normal en ese punto de hit, y el vector
   desde la fuente de luz hasta ese punto de hit.
3. Luego cada material toma en cuenta estos tres vectores para producir el
   color especifico de este mismo en ese punto.
4. Coloreamos la posicion en la que hay hit (la coloreamos en canvas space) con
   el color dado.
