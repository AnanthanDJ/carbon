import { motion, type MotionProps } from "motion/react";
import { useEffect, useState } from "react";

interface AnimatedSpanProps extends MotionProps {
  children: React.ReactNode;
  delay?: number;
  className?: string;
}

function AnimatedSpan({
  children,
  delay = 0,
  className,
  ...props
}: AnimatedSpanProps) {
  return (
    <motion.div
      initial={{ opacity: 0, y: -4 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{
        duration: 0.3,
        delay: delay / 1000,
      }}
      className={className}
      {...props}
    >
      {children}
    </motion.div>
  );
}

interface TypingAnimationProps extends MotionProps {
  children: string;
  delay?: number;
  duration?: number;
  className?: string;
}

function TypingAnimation({
  children,
  delay = 0,
  duration = 45,
  className,
}: TypingAnimationProps) {
  const [text, setText] = useState("");
  const [started, setStarted] = useState(false);

  useEffect(() => {
    const timeout = setTimeout(() => setStarted(true), delay);

    return () => clearTimeout(timeout);
  }, [delay]);

  useEffect(() => {
    if (!started) return;

    let i = 0;

    const timer = setInterval(() => {
      if (i < children.length) {
        setText(children.slice(0, i + 1));
        i++;
      } else {
        clearInterval(timer);
      }
    }, duration);

    return () => clearInterval(timer);
  }, [children, duration, started]);

  return <div className={className}>{text}</div>;
}

export function HeroTerminal() {
  return (
    <div className="hero-terminal">
      <div className="hero-terminal__titlebar">
        <span />
        <span />
        <span />
      </div>

      <pre className="hero-terminal__content">
        <TypingAnimation delay={300}>$ echo "Hello, Terminal!"</TypingAnimation>

        <AnimatedSpan delay={1700}>Hello, Terminal!</AnimatedSpan>

        <TypingAnimation delay={2600}>$ pwd</TypingAnimation>

        <AnimatedSpan delay={3600}>/home/student</AnimatedSpan>

        <TypingAnimation delay={4300}>$ ls</TypingAnimation>

        <AnimatedSpan delay={5200}>Documents Downloads notes.txt</AnimatedSpan>

        <TypingAnimation delay={6100}>$ mkdir terminal-camp</TypingAnimation>

        <TypingAnimation delay={7600}>$</TypingAnimation>
      </pre>
    </div>
  );
}
