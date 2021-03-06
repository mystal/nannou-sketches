// From: https://dailygenerative.art.blog/2020/01/18/spill/
// Reddit discussion: https://www.reddit.com/r/generative/comments/eqjdd9/spill/

// made by alex
// iamalexbaker@gmail.com
// dailygenerative.art.blog
 
// KEY COMMANDS
// ENTER - Start everything again (new setup)
// Space - save screenshot
// Z     - show zones/planets (for debugging)
// F     - freeze all particles
// P     - enable/disable planets
// A     - give all particles a velocity boost
// T     - enable/disable real time mode
// R     - make all particles return to centre
// V     - manually toggle recording
// X     - enable/disable smart recording - if this is on, recording will auto start with you press enter and auto start when particles return
 
import com.hamoid.*; // using Video Export library
boolean recording = false;
VideoExport videoExport;
boolean smartRecording = false;
 
float drag = 0.999; // set to 1 for no drag
float gravity = 0.;
float powerRange = 0.02; // for zones
float[] sizeRange = {100, 400}; // for zones/planets
float circleRadius = 300;
float circleAngle = PI/128;
int initialiseMode = 3; // 0: random, 1: circle, 2: polar rose, 3: n-gon
float[] initialImpulse = {0.001, 0.01, 0}; // good range: 0.01 - 0.035
boolean randomImpulses = true; // if true, each particle has a different starting velocity
float startDirection = -1; // +1: fly away from circle, -1: fly into it
float shape = 5;
float[] startAngle = {0, TWO_PI, 0};
float[] planetMass = {1.0, 2.5};
float systemPull = 0.2;
int systemPullMode = 2; // 0 to drag into centre or 2 to pull into orbit
float systemCentre = 1.0;
boolean boundaries = false; // stop things going off the edge of the screen
float particleSize = 1;
int numParticles = 3000;
int numZones = 0;
int numPlanets = 10;
int numTicks = 1000;
int staticTicks = 1000;
int realTimeTicks = 3;
boolean realTimeColour = true;
float returnTurnRate = 0.2;
float returnAccel = 1.0008;
float returnSnapDistance = 5;
boolean dissipate = false; // this doesn't really work as intended, I was trying to reacreate a cool looking bug
float freezeRate = 0.05;
float accelAmt = 1.1;
float[] zoneStrength = {0.01, 0.1};
 
 
boolean showZones = false;
boolean do_draw = true;
boolean realTime = false;
boolean clearScreen = true; // if true, clears the screen each time the system is reset (press enter). (non realtime mode only)
 
float[] hueRange = {0, TWO_PI};
float[] satRange = {0.5, 1.0};
float[] briRange = {0.5, 1.0};
float alpha = 15;
int nc = 2;
color[] c = new color[nc];
color bgc;
float hue, sat, bri;
 
ArrayList<particle> particles;
ArrayList<zone> zones;
ArrayList<planet> planets;
ArrayList<PVector> traces;
int currentTick, currentTickRT;
boolean returning, frozen = false;
boolean enablePlanets = true;
 
boolean initialised = false;
 
void setup(){
  size(800, 800);
  if(!initialised){
    videoExport = new VideoExport(this);
    videoExport.setDebugging(false);
     
    colorMode(HSB, TWO_PI, 1, 1);
    bgc = color(0, 0, 0.2);
    background(bgc);
    initialised = true;
  }
   
  if(clearScreen) background(bgc);
   
  hue = random(hueRange[0], hueRange[1]);
  sat = random(satRange[0], satRange[1]) * TWO_PI;
  bri = random(briRange[0], briRange[1]) * TWO_PI;
  for(int i = 0; i < nc; i++){
    c[i] = color(random(hueRange[0], hueRange[1]), random(satRange[0], satRange[1]), random(briRange[0], briRange[1]));
  }
   
  currentTick = 0;
  returning = false;
  frozen = false;
  initialImpulse[2] = random(initialImpulse[0], initialImpulse[1]);
  startAngle[2] = random(startAngle[0], startAngle[1]);
  particles = new ArrayList<particle>();
  zones = new ArrayList<zone>();
  planets = new ArrayList<planet>();
  traces = new ArrayList<PVector>();
  PVector origin = new PVector(width/2, height/2);
  if(initialiseMode == 0){
    for(int i = 0; i < numParticles; i++){
      if(randomImpulses) initialImpulse[2] = random(initialImpulse[0], initialImpulse[1]);
      PVector force = PVector.random2D().mult(initialImpulse[2]);
      particles.add(new particle(random(width), random(height), force.x*50, force.y*50));
    }
  }
  if(initialiseMode == 1){
    for(int i = 0; i < numParticles; i++){
      float r = circleAngle * i;
      float xp = circleRadius * cos(r);
      float yp = circleRadius * sin(r);
      if(randomImpulses) initialImpulse[2] = random(initialImpulse[0], initialImpulse[1]);
      particles.add(new particle(origin.x+xp, origin.y+yp, xp * initialImpulse[2] * startDirection, yp * initialImpulse[2] * startDirection));
    }
  }
  if(initialiseMode == 2){
    float n = TWO_PI / numParticles;
    for(int i = 0; i < numParticles; i++){
      float t = n * i;
      float r = cos(shape * t) * circleRadius;
      float xp = r * cos(t);
      float yp = r * sin(t);
      if(randomImpulses) initialImpulse[2] = random(initialImpulse[0], initialImpulse[1]);
      particles.add(new particle(origin.x+xp, origin.y+yp, xp * initialImpulse[2] * startDirection, yp * initialImpulse[2] * startDirection));
    }
  }
  if(initialiseMode == 3){
    float bigt = TWO_PI / shape;
    float crd2 = sin(bigt / 2);
    float n = TWO_PI / numParticles;
    for(int i = 0; i < numParticles; i++){
      float t = startAngle[2] + n * i;
      float tt = (startAngle[2] + t) % bigt;
      float r = crd2 / (tan(abs(bigt/2)) * cos(abs(bigt/2 - tt)));
      float xp = circleRadius * r * cos(t);
      float yp = circleRadius * r * sin(t);
      if(randomImpulses) initialImpulse[2] = random(initialImpulse[0], initialImpulse[1]);
      particles.add(new particle(origin.x+xp, origin.y+yp, xp * initialImpulse[2] * startDirection, yp * initialImpulse[2] * startDirection));
    }
  }
   
  if(systemCentre > 0) planets.add(new planet(origin.x, origin.y, systemCentre, 5, 1));
  if(systemPull > 0) planets.add(new planet(origin.x, origin.y, width, systemPull, systemPullMode));
  for(int j = 0; j < numPlanets; j++){
    planets.add(new planet(random(width),random(height),random(sizeRange[0], sizeRange[1]),random(planetMass[0], planetMass[1]),2));
  }
  for(int k = 0; k < numZones; k++){
    zones.add(new zone(random(width),random(height),random(-powerRange/2,powerRange/2),random(-powerRange/2,powerRange/2),random(sizeRange[0], sizeRange[1]),true, random(zoneStrength[0], zoneStrength[1])));
  }
  do_draw = true;
   
  if(smartRecording && !recording) setRecording(true); 
}
 
void draw(){
  if(realTime){
    background(bgc);
    traces = new ArrayList<PVector>();
    numTicks = realTimeTicks;
    if(!realTimeColour) currentTick = 0;
    do_draw = true;
  } else numTicks = staticTicks;
  if(!do_draw) return;
  for(int k = 0; k < numTicks; k++){
    if(enablePlanets){
      for(planet pl: planets){
        pl.tick();
        if(showZones) pl.show();
      }
      for(zone z: zones){
        z.tick();
        if(showZones) z.show();
      }
    }
    for(particle p: particles){
      p.tick();
      //p.show();
    }
    currentTick++;
  }
  noStroke();
  //ellipse(400,400,200,200);
  //println(traces.size());
  for(PVector t: traces){
    fill(lerpColor(c[0], c[1], t.z), alpha);
    ellipse(t.x, t.y, particleSize, particleSize);
  }
  do_draw = false;
   
  if (recording) {
    videoExport.saveFrame();
  }
   
  if(smartRecording && returning && recording){
    int h = 0;
    for(particle p: particles){
      if(p.home) h++;
    }
    if(h >= particles.size()) setRecording(false);
  }
}
 
class particle{
  PVector initial, pos, vel;
  boolean home;
   
  particle(float x, float y, float vx, float vy){
    pos = new PVector(x, y); initial = new PVector(x, y); vel = new PVector(vx, vy); home = false;
  }
   
  void tick(){
    pos.x += vel.x;
    pos.y += vel.y + gravity;
    if(boundaries){
      if(pos.x < 0 || pos.x > width) vel.x = -vel.x;
      if(pos.y < 0 || pos.y > height) vel.y = -vel.y;
    }
    if(!returning) vel.mult(drag);
    else goHome();
    if(frozen) freeze();
    traces.add(new PVector(pos.x, pos.y, float(currentTick)/staticTicks));
  }
   
  void goHome(){
    if(pos.dist(initial) > returnSnapDistance || dissipate){
      PVector rV = new PVector(initial.x - pos.x, initial.y - pos.y).normalize();
      float mag = vel.mag();
      vel.normalize();
      vel.lerp(rV, returnTurnRate);
      vel.setMag(mag*returnAccel);
      //drawVector(pos, vel, 5, #FF0000);
      //drawVector(pos, rV, 5, #FFFFFF);
    }
    if(pos.dist(initial) < returnSnapDistance){
      pos.x = initial.x;
      pos.y = initial.y;
      home = true;
    }
     
  }
   
  void freeze(){
    PVector rV = new PVector(0, 0);
    vel.lerp(rV, freezeRate);
  }
   
  void randomVelocity(){
    if(randomImpulses) initialImpulse[2] = random(initialImpulse[0], initialImpulse[1]);
    vel = PVector.random2D().mult(initialImpulse[2]*50);
  }
   
  void show(){
    stroke(50, 150);
    noFill();
    ellipse(pos.x, pos.y, particleSize, particleSize);
  }
}
 
class planet{
  PVector pos;
  float radius, mass;
  int mode; // 0: black hole, 1: push away, 2: orbit
  planet(float x, float y, float r, float m, int mm){
    pos = new PVector(x, y); radius = r; mass = m; mode = mm;
  }
   
  void tick(){
    attractParticles();
  }
     
  void show(){
    noStroke();
    fill(3*PI/2, 0.7, 0.7, 100);
    ellipse(pos.x, pos.y, radius/2, radius/2);
  }
   
  void attractParticles(){
    for(particle p: particles){
      if(pos.dist(p.pos) <= radius/2){
        PVector facingPlanet = new PVector(pos.x - p.pos.x , pos.y - p.pos.y).normalize();
        float velMag = p.vel.mag();
        p.vel.normalize();
        switch(mode){
          case 0:
             
            //stroke(0,0,1,100);
            //line(p.pos.x, p.pos.y, p.pos.x + p.vel.x*100, p.pos.y + p.vel.y*100);
            //stroke(PI/4,0.5,1,100);
            //line(p.pos.x, p.pos.y, p.pos.x + facingPlanet.x*100, p.pos.y + facingPlanet.y*100);
            p.vel.lerp(facingPlanet, mass*0.1);
            break;
          case 1:
          facingPlanet.mult(-1);
            p.vel.lerp(facingPlanet, mass*0.1);
            break;
          case 2:
            PVector tangent = new PVector(facingPlanet.x, facingPlanet.y).rotate(PI/(2*mass));
            //stroke(0,0,1,100);
            //line(p.pos.x, p.pos.y, p.pos.x + p.vel.x*50, p.pos.y + p.vel.y*50);
            //stroke(PI/4,0.9,1,100);
            //line(p.pos.x, p.pos.y, p.pos.x + tangent.x*50, p.pos.y + tangent.y*50);
            p.vel.lerp(tangent,mass*0.03);
            break;
        }
        p.vel.setMag(velMag);
      }
    }
  }
}
 
class zone{
  PVector pos, force;
  float radius;
  boolean mode; // false = apply force, true = lerp to direction
  float strength;
  zone(float x, float y, float fx, float fy, float r, boolean m, float s){
    pos = new PVector(x, y); force = new PVector(fx, fy); radius = r; mode = m; if(mode) force = PVector.random2D(); strength = s;
  }
   
  void tick(){
    pushParticles();
  }
   
  void show(){
    stroke(PI, 100);
    line(pos.x, pos.y, pos.x + force.x*500, pos.y + force.y * 500);
    fill(50,80,200, 50);
    noStroke();
    ellipse(pos.x, pos.y, radius/2, radius/2);
     
  }
   
  void pushParticles(){
    for(particle p: particles){
      if(pos.dist(p.pos) <= radius/2){
        if(mode){
          float mag = p.vel.mag();
          p.vel.normalize().lerp(force, strength).setMag(mag);
        }else{
          p.vel.add(force);
        }
      }
    }
  }
}
 
void drawVector(PVector pt, PVector v, float m, color col){
  stroke(col);
  line(pt.x, pt.y, pt.x + v.x*m, pt.y + v.y*m);
}
 
void setRecording(boolean rec){
  recording = rec;
  if(recording == true){
    videoExport.setMovieFileName(hour()+"-"+minute()+"-"+second() + ".mp4");
    videoExport.startMovie();
    println("Started recording");
  }
  else{
    println("Finished recording");
    videoExport.endMovie();
  }
}
 
void keyPressed()
{
  if (keyCode==32) { // space
    String fn = "pg-"+hour()+"-"+minute()+"-"+second()+".png";
    saveFrame(fn);
    println("Saved image "+fn);
  }
  if (keyCode==10) { // enter
    setup();
    println("Initialising new state");
  }
  if(keyCode==90) { // Z
    showZones = !showZones;
    println("Show Zones: "+showZones);
  }
  if(keyCode==82) { // R
    returning = !returning;
    println("Returning: "+returning);
    if(!returning){
      for(particle p: particles){
        p.home = false;
      }
    }
  }
  if(keyCode==70) { // F
    frozen = !frozen;
    println("Frozen: "+frozen);
    if(!frozen){
      initialImpulse[2] = random(initialImpulse[0], initialImpulse[1]);
      for(particle p: particles){
        p.randomVelocity();
      }
    }
  }
  if(keyCode==80) { // P
    enablePlanets = !enablePlanets;
    println("Enable planets: "+enablePlanets);
  }
  if(keyCode==65) { // A
    for(particle p: particles){
      p.vel.mult(accelAmt);
    }
  }
  if(keyCode==86){ // V
    setRecording(!recording);
  }
  if(keyCode==88){ // X
    smartRecording = !smartRecording;
    println("Smart Recording: "+smartRecording);
  }
  if(keyCode==84) // T
  {
    realTime = !realTime;
    println("Real Time Mode: "+realTime);
  }
}
